mod notifications;
mod organization;
mod settings;

use std::{sync::mpsc::Sender, thread, time::Duration};

use chrono::{DateTime, Local};
use notifications::show_files_count_notification;
use organization::OrganizationCommand;
use specta_typescript::Typescript;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    App, AppHandle, Manager, WindowEvent,
};
use tauri_plugin_cli::CliExt;
use tauri_specta::*;
use tokio::sync::Mutex;

use crate::notifications::show_files_count_notification_wo_settings;

#[tauri::command]
#[specta::specta]
fn load_settings_from_file(app_handle: AppHandle) -> settings::SettingsStruct {
    settings::load_settings_from_file(&app_handle)
}

#[tauri::command]
#[specta::specta]
fn save_settings_to_file(
    app_handle: AppHandle,
    settings: settings::SettingsStruct,
) -> std::result::Result<(), ()> {
    settings::save_settings_to_file(&app_handle, &settings).map_err(|_| {})?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
fn fetch_app_process_log(app_handle: AppHandle) -> std::result::Result<Vec<String>, ()> {
    let state = app_handle.state::<ProcessLogState>();
    let log = state.log.blocking_lock();
    Ok((*log).to_vec())
}

fn setup_tray_icon(app: &mut App) -> anyhow::Result<TrayIcon> {
    const MENU_ID_QUIT: &str = "quit";
    let menu_quit = MenuItem::with_id(app, MENU_ID_QUIT, "終了", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&menu_quit])?;

    let app_name = match &app.config().product_name {
        Some(value) => value,
        None => "VRC Pictures Organizer",
    };

    let tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip(app_name)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            MENU_ID_QUIT => {
                stop_watching_process(app);
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                // トレイアイコンを左クリック: メインウィンドウを表示する
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
                // 画像整理スレッドを停止
                stop_watching_process(app);
            }
            _ => {}
        })
        .build(app)?;
    Ok(tray)
}

struct ProcessLogState {
    log: Mutex<Vec<String>>,
}

fn add_process_log(app: &AppHandle, message: &str) {
    let date_time: DateTime<Local> = Local::now();
    let date_time_str = date_time.format("%F %H:%M:%S").to_string();

    let state = app.state::<ProcessLogState>();
    let mut log = state.log.blocking_lock();
    (*log).insert(0, format!("[{}] {}", date_time_str, message));
}

struct OrganizationState {
    thread_sender: Mutex<Option<Sender<OrganizationCommand>>>,
}

fn start_watching_process(app: &AppHandle) {
    let outcome = organization::start_organize_thread(app);
    if let Ok(outcome) = outcome {
        // コマンド送信用channel
        let state = app.state::<OrganizationState>();
        let mut sender = state.thread_sender.blocking_lock();
        *sender = Some(outcome.0);

        // 処理件数受け取り用channel
        let app_handle = app.clone();
        thread::spawn(move || {
            while let Ok(file_count) = outcome.1.recv() {
                if file_count == -1 {
                    // -1はTerminate時に送信される
                    break;
                } else {
                    show_files_count_notification(&app_handle, &file_count);
                    add_process_log(&app_handle, &format!("ファイル処理完了: {}件", &file_count));
                }
            }
        });
    }
}

fn stop_watching_process(app: &AppHandle) {
    let state = app.state::<OrganizationState>();
    let mut sender = state.thread_sender.blocking_lock();
    if let Some(some_sender) = sender.as_mut() {
        let _ = some_sender.send(OrganizationCommand::Terminate);
        *sender = None;
    }
}

// === run() ===

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let invoke_builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(collect_commands![
            load_settings_from_file,
            save_settings_to_file,
            fetch_app_process_log,
        ]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    invoke_builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(invoke_builder.invoke_handler())
        .setup(|app| {
            // トレイアイコン
            let _ = setup_tray_icon(app)?;
            // 実行ログ
            app.manage(ProcessLogState {
                log: Mutex::new(vec![]),
            });
            add_process_log(app.app_handle(), "アプリ起動");

            // コマンドライン引数解析
            let is_oneshot = match app.cli().matches() {
                Ok(matches) => match matches.args.get("oneshot") {
                    Some(arg) => {
                        if let Some(val) = arg.value.as_bool() {
                            val
                        } else {
                            false
                        }
                    }
                    None => false,
                },
                Err(_e) => false,
            };
            println!("is_oneshot: {:?}", is_oneshot);

            // 画像整理スレッドを起動
            if is_oneshot {
                // ワンショット起動（一度だけ処理してアプリ終了）
                let app = app.app_handle();
                let outcome = organization::start_organize_thread(app);
                if let Ok(outcome) = outcome {
                    app.manage(OrganizationState {
                        thread_sender: Mutex::new(Some(outcome.0)),
                    });
                    // 処理件数受け取り用channel
                    let app_handle = app.clone();
                    thread::spawn(move || {
                        while let Ok(file_count) = outcome.1.recv() {
                            if file_count == -1 {
                                // -1はTerminate時に送信される
                                break;
                            } else {
                                // ワンショット時は設定に関わらず通知を出す
                                show_files_count_notification_wo_settings(&app_handle, &file_count);
                            }
                            // 通知を出す前にアプリが終了しないよう、スレッドを短時間一時停止する
                            thread::sleep(Duration::from_millis(500));
                            stop_watching_process(&app_handle);
                            app_handle.exit(0);
                        }
                    });
                }
            } else {
                // 通常起動（バックグラウンドで動作）
                let outcome = organization::start_organize_thread(app.app_handle());
                if let Ok(outcome) = outcome {
                    app.manage(OrganizationState {
                        thread_sender: Mutex::new(Some(outcome.0)),
                    });
                    // 処理件数受け取り用channel
                    let app_handle = app.app_handle().clone();
                    thread::spawn(move || {
                        while let Ok(file_count) = outcome.1.recv() {
                            if file_count == -1 {
                                // -1はTerminate時に送信される
                                break;
                            } else {
                                show_files_count_notification(&app_handle, &file_count);
                                add_process_log(
                                    &app_handle,
                                    &format!("ファイル処理完了: {}件", &file_count),
                                );
                            }
                        }
                    });
                }
            }
            Ok(())
        })
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                // ウィンドウを閉じる操作の場合、アプリを終了するのではなく
                // ウィンドウを見えなくする
                window.hide().unwrap();
                api.prevent_close();
                // 画像整理スレッドを起動
                start_watching_process(window.app_handle());
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
