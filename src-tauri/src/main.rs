// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{DateTime, Local, NaiveTime, TimeZone};
use fs_extra::file::write_all;
use serde_json::{json, Value};
use tauri::api::dir::{self, DiskEntry};
use tauri::api::file::read_string;
use tauri::api::path;
use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayEvent, SystemTrayMenu};

/// システムトレイインスタンスを生成して返す
fn create_system_tray_menu() -> SystemTray {
    let menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("settings".to_string(), "設定"))
        .add_item(CustomMenuItem::new("github".to_string(), "GitHub"))
        .add_item(CustomMenuItem::new("quit".to_string(), "終了"));
    SystemTray::new().with_menu(menu)
}

use tauri::Manager;

/// メインウィンドウ（設定画面）を開く
///
/// * `app` - AppHandle
fn show_settings_window(app: &tauri::AppHandle) {
    let _ = app.get_window("main").map(|win| win.show());
    // 設定画面を表示中は監視を止める
    stop_watching(app.state::<WatchingState>())
}

/// アプリ設定構造体
#[derive(serde::Serialize)]
struct Setting {
    /// フォルダ監視間隔（秒）
    interval: u16,
    /// 日付が変わったと判断する時刻（HH:MM）
    date_line: String,
}
/// アプリ設定state
struct SettingState(Mutex<Setting>);

/// 設定ファイルを読み込み、設定値を返す
/// 無い場合はデフォルト値を返す
fn get_setting(app: &tauri::AppHandle) -> Setting {
    let config_dir_path = app.path_resolver().app_config_dir();
    if let Some(config_dir_path) = config_dir_path {
        let config_file_path = config_dir_path.join("settings.json");
        if config_file_path.exists() {
            let file = read_string(config_file_path);
            if let Ok(file_str) = file {
                let setting = serde_json::from_str::<Value>(&file_str);
                if let Ok(setting) = setting {
                    return Setting {
                        interval: u16::try_from(setting["interval"].as_u64().unwrap_or(30))
                            .unwrap_or(30),
                        date_line: setting["date_line"].as_str().unwrap_or("12:00").to_string(),
                    };
                }
            }
        }
    }

    Setting {
        interval: 10,
        date_line: "12:00".to_string(),
    }
}

/// フロント用に現在の設定を返す
#[tauri::command]
fn get_setting_for_screen(setting_state: State<SettingState>) -> Result<Setting, ()> {
    let state = setting_state.0.lock();
    match state {
        Ok(state) => Ok(Setting {
            interval: state.interval,
            date_line: state.date_line.to_owned(),
        }),
        Err(_) => Err(()),
    }
}

/// フロントから現在の設定を保存する
#[tauri::command(rename_all = "snake_case")]
fn save_setting_for_screen(
    interval: u16,
    date_line: String,
    setting_state: State<SettingState>,
    app_handle: tauri::AppHandle,
) -> Result<(), ()> {
    let state = setting_state.0.lock();
    match state {
        Ok(mut state) => {
            state.interval = interval;
            state.date_line = date_line;

            // 保存処理
            // 保存先は {Windowsのユーザーフォルダ}\AppData\Roaming\jp.nano2.vrc-pictures-organizer
            let config_dir_path = app_handle.path_resolver().app_config_dir();
            if let Some(config_dir_path) = config_dir_path {
                let config_file_path = config_dir_path.join("settings.json");
                let json = json!({
                  "interval": state.interval,
                  "date_line": state.date_line
                });
                let json_str = serde_json::to_string(&json);
                if let Ok(json_str) = json_str {
                    let _ = write_all(config_file_path, &json_str);
                }
            }

            Ok(())
        }
        Err(_) => Err(()),
    }
}

/// 処理ログstate
struct LogState(Mutex<Vec<String>>);

/// ログを追加する
/// 新しいログはベクタの先頭に挿入される
fn add_log(app_handle: &tauri::AppHandle, message: &str) {
    let binding = app_handle.state::<LogState>();
    let log_state = binding.0.lock();
    if let Ok(mut log_state) = log_state {
        let date_time: DateTime<Local> = Local::now();
        let date_time_str = date_time.format("%F %H:%M:%S").to_string();
        log_state.insert(0, format!("[{}] {}", date_time_str, message));
    }
}

/// フロント用に現在のログを返す
#[tauri::command]
fn get_log_for_screen(log_state: State<LogState>) -> Result<Vec<String>, ()> {
    let state = log_state.0.lock();
    match state {
        Ok(state) => Ok(state.to_vec()),
        Err(_) => Err(()),
    }
}

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fs, thread};
use tauri::State;

/// フォルダ監視状態state
struct WatchingState(Arc<AtomicBool>);

/// 監視を停止する
#[tauri::command]
fn stop_watching(watch_state: State<WatchingState>) {
    println!("stop_watching");
    watch_state.0.store(true, Ordering::Relaxed);
}

/// 監視を開始する
#[tauri::command]
fn start_watching(app_handle: tauri::AppHandle) {
    println!("start_watching");
    tauri::async_runtime::spawn(async move {
        let watching_state = &app_handle.state::<WatchingState>().0;
        watching_state.store(false, Ordering::Relaxed);
        let setting_state = &app_handle.state::<SettingState>().0;
        loop {
            if watching_state.load(Ordering::Relaxed) {
                println!("... stop");
                break;
            }
            println!("... running");
            let mut move_count = 0;
            let target_files = extract_target_files();
            for target_file in target_files {
                let target_file = &target_file;
                // 新しい親ディレクトリを生成
                let new_parent_directory = target_file.path.parent();
                if let Some(new_parent_directory) = new_parent_directory {
                    let new_directory = judge_new_folder_from_modified(
                        target_file,
                        &setting_state.lock().unwrap().date_line,
                    );
                    if let Ok(new_directory) = new_directory {
                        let new_parent_directory = &new_parent_directory.join(new_directory);
                        if !new_parent_directory.exists() {
                            // フォルダがなければ生成する
                            let _ = fs::create_dir(new_parent_directory);
                        }

                        let file_name = target_file.path.file_name();
                        match file_name {
                            Some(file_name) => {
                                let new_file_path = new_parent_directory.join(file_name);
                                // ファイル移動
                                let result = fs_extra::file::move_file(
                                    &target_file.path,
                                    new_file_path,
                                    &fs_extra::file::CopyOptions::new(),
                                );
                                if result.is_ok() {
                                    move_count += 1;
                                };
                            }
                            None => {}
                        }
                    }
                }
            }
            println!("file moved: {}", move_count);
            add_log(&app_handle, &format!("ファイル処理完了: {}件", move_count));

            let interval = setting_state.lock();
            let interval = match interval {
                Ok(interval) => interval.interval,
                Err(_) => 30,
            };
            thread::sleep(Duration::from_secs(interval.into()));
        }
    });
}

/// VRChatの画像フォルダから、移動対象になるファイルを抽出する
fn extract_target_files() -> Vec<DiskEntry> {
    // VRChatの画像が保存されるフォルダは {Windowsのピクチャフォルダ}/VRChat
    let vrc_folder_path = path::picture_dir().and_then(|res| Some(res.join("VRChat")));
    match vrc_folder_path {
        Some(path) => {
            // {Windowsのピクチャフォルダ}/VRChat 以下を再帰的に取得
            let entries = dir::read_dir(path, true);
            match entries {
                Ok(ok_entries) => {
                    let mut result: Vec<DiskEntry> = Vec::new();
                    // 直下には年月単位のフォルダしかないので、その下の層（children）を見にいく
                    for yyyy_mm in ok_entries {
                        match yyyy_mm.children {
                            Some(yyyy_mm_item) => {
                                result.append(&mut extract_files(yyyy_mm_item));
                            }
                            _ => {}
                        }
                    }
                    result
                }
                Err(_) => Vec::<DiskEntry>::new(),
            }
        }
        _ => Vec::<DiskEntry>::new(),
    }
}

/// フォルダ情報の入ったベクタから、ファイルを抽出する
fn extract_files(items: Vec<DiskEntry>) -> Vec<DiskEntry> {
    let mut result: Vec<DiskEntry> = Vec::new();
    for item in items {
        let meta = item.path.metadata();
        if let Ok(meta) = meta {
            // ファイルの場合のみ抽出
            if meta.is_file() {
                result.push(item);
            }
        }
    }
    result
}

/// 新しく格納するフォルダの名前をファイルの更新日時（生成日時）を基準に決定して返す
fn judge_new_folder_from_modified(entry: &DiskEntry, date_line: &String) -> Result<String, ()> {
    let metadata = entry.path.metadata();
    match metadata {
        Ok(metadata) => {
            // ファイルの更新日時（生成日時）をUnixTimeで取得
            let modified_unixtime = metadata.modified();
            let modified_unixtime = match modified_unixtime {
                Ok(modified_unixtime) => {
                    match modified_unixtime.duration_since(std::time::UNIX_EPOCH) {
                        Ok(modified_unixtime) => modified_unixtime,
                        Err(_) => return Err(()),
                    }
                }
                Err(_) => return Err(()),
            };
            let modified_unixtime_secs: i64 = match modified_unixtime.as_secs().try_into() {
                Ok(modified_unixtime_secs) => modified_unixtime_secs,
                Err(_) => return Err(()),
            };
            let modified_datetime = match Local.timestamp_opt(modified_unixtime_secs, 0) {
                chrono::LocalResult::Single(modified_datetime) => modified_datetime,
                _ => return Err(()),
            };

            // 比較用の基準時刻情報を生成
            // 設定情報の日付変更線時刻は HH:MM 形式なので、":" で区切る
            let date_line_vec: Vec<&str> = date_line.split(':').collect();
            let hour = date_line_vec[0].parse::<u32>().unwrap_or(12);
            let minute = date_line_vec[1].parse::<u32>().unwrap_or(0);
            let date_line_time = match NaiveTime::from_hms_opt(hour, minute, 0) {
                Some(date_line_time) => date_line_time,
                None => return Err(()),
            };

            // 新しいフォルダ名判定
            let new_folder_name = if modified_datetime.time() < date_line_time {
                // 日付変更線とする時間よりも前の場合、前日扱い
                modified_datetime.date_naive() - chrono::Duration::days(1)
            } else {
                modified_datetime.date_naive()
            };

            Ok(new_folder_name.to_string())
        }
        Err(_) => Err(()),
    }
}

fn main() {
    tauri::Builder::default()
        .system_tray(create_system_tray_menu())
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                // 設定画面を開く
                show_settings_window(app);
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    // アプリを終了
                    let _ = app.tray_handle().destroy();
                    std::process::exit(0);
                }
                "github" => {
                    let _ = tauri::api::shell::open(
                        &app.shell_scope(),
                        "https://github.com/nano-nano/vrc-pictures-organizer",
                        None,
                    );
                }
                "settings" => {
                    // 設定画面を開く
                    show_settings_window(app);
                }
                _ => {}
            },
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                // ウィンドウを閉じる操作の場合、アプリを終了するのではなく
                // ウィンドウを見えなくする
                event.window().hide().unwrap();
                api.prevent_close();
                // 監視を再開
                start_watching(event.window().app_handle());
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            start_watching,
            stop_watching,
            get_setting_for_screen,
            save_setting_for_screen,
            get_log_for_screen
        ])
        .setup(|app| {
            // 設定ファイル参照
            let settings = get_setting(&app.app_handle());
            app.manage(SettingState(Mutex::new(settings)));

            // 監視状態初期化
            let is_stop_watching = Arc::new(AtomicBool::new(false));
            app.manage(WatchingState(is_stop_watching));

            // ログ初期化
            app.manage(LogState(Mutex::new(Vec::new())));
            add_log(&app.app_handle(), "アプリ起動");

            // 監視開始
            start_watching(app.app_handle());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
