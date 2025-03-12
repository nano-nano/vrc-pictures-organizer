use std::{
    fs::{create_dir, DirEntry},
    path::PathBuf,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

use chrono::{Local, NaiveTime, TimeZone};
use tauri::{AppHandle, Manager};

use crate::settings::{load_settings_from_file, SettingsStruct};

pub enum OrganizationCommand {
    Terminate,
}

pub fn start_organize_thread(
    app_handle: &AppHandle,
) -> anyhow::Result<(Sender<OrganizationCommand>, Receiver<i8>)> {
    let (sender_command, receiver_command) = channel::<OrganizationCommand>();
    let (sender_file_count, receiver_file_count) = channel::<i8>();
    let settings = load_settings_from_file(app_handle);
    // VRChatの画像が保存されるフォルダは {Windowsのピクチャフォルダ}/VRChat
    let vrc_folder_path = app_handle.path().picture_dir()?.join("VRChat");
    thread::spawn(move || {
        watching_thread(
            receiver_command,
            sender_file_count,
            settings,
            &vrc_folder_path,
        )
    });
    Ok((sender_command, receiver_file_count))
}

fn watching_thread(
    receiver: Receiver<OrganizationCommand>,
    sender: Sender<i8>,
    settings: SettingsStruct,
    vrc_folder_path: &PathBuf,
) {
    let interval_sec = settings.get_interval_sec();
    let date_line = settings.get_date_line();

    let mut interval_counter: u8 = interval_sec;
    loop {
        // コマンドを受信していない場合はブロックして欲しくないので try_recv
        if let Ok(command) = receiver.try_recv() {
            match command {
                OrganizationCommand::Terminate => {
                    println!("thread terminate");
                    let _ = sender.send(-1);
                    break;
                }
            }
        };
        println!("thread working");
        if interval_counter == interval_sec {
            println!("organize run");
            let mut moved_file_count = 0;
            let target_file_entries = extract_target_files(vrc_folder_path);
            for target_file_entry in target_file_entries {
                let target_file_path = target_file_entry.path();
                let target_file_name = target_file_entry.file_name();
                let new_folder_name =
                    judge_new_folder_from_modified(&target_file_entry, &date_line);
                if let (Some(parent), Ok(new_folder_name)) =
                    (target_file_path.parent(), new_folder_name)
                {
                    // 親ディレクトリと、新しい振り分け先のフォルダが決まったら振り分け処理を実施
                    let new_parent_directory = parent.join(new_folder_name);
                    if !new_parent_directory.exists() {
                        // 振り分け先フォルダがない場合は作成する
                        let _ = create_dir(&new_parent_directory);
                    }

                    // ファイルを移動
                    let new_file_path = &new_parent_directory.join(&target_file_name);
                    let outcome = fs_extra::file::move_file(
                        &target_file_path,
                        new_file_path,
                        &fs_extra::file::CopyOptions::new(),
                    );
                    if outcome.is_ok() {
                        // 処理成功の件数をカウント
                        moved_file_count += 1;
                    }
                }
            }
            println!("organize end: {} files moved.", moved_file_count);
            let _ = sender.send(moved_file_count);

            interval_counter = 1;
        } else {
            interval_counter += 1;
        }
        thread::sleep(Duration::from_secs(1));
    }
}

/// VRChatの画像フォルダから、移動対象になるファイルを抽出する
fn extract_target_files(vrc_folder_path: &PathBuf) -> Vec<DirEntry> {
    let mut file_entries: Vec<DirEntry> = vec![];
    let entries = match vrc_folder_path.read_dir() {
        Ok(value) => value,
        Err(_) => return vec![],
    };
    for entry in entries {
        match entry {
            Ok(entry) => {
                // 直下には年月単位のフォルダしかないので、その中を見にいく
                let child_entries = match entry.path().read_dir() {
                    Ok(value) => value,
                    Err(_) => return vec![],
                };
                for child_entry in child_entries {
                    match child_entry {
                        Ok(child_entry) => {
                            // ディレクトリ判定
                            let is_dir = match child_entry.metadata() {
                                Ok(value) => value.is_dir(),
                                Err(_) => false,
                            };
                            if !is_dir {
                                file_entries.push(child_entry);
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
            Err(_) => {}
        }
    }
    file_entries
}

/// 新しく格納するフォルダの名前をファイルの更新日時（生成日時）を基準に決定して返す
fn judge_new_folder_from_modified(
    target_file_entry: &DirEntry,
    date_line: &str,
) -> anyhow::Result<String> {
    // ファイルの更新日時（生成日時）をUnixTimeで取得
    let metadata = target_file_entry.metadata()?;
    let modified_unixtime_secs: i64 = metadata
        .modified()?
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs()
        .try_into()?;
    let modified_datetime = match Local.timestamp_opt(modified_unixtime_secs, 0) {
        chrono::offset::LocalResult::Single(value) => value,
        _ => return Err(anyhow::anyhow!("")),
    };

    // 比較用の基準時刻情報を生成
    // 設定情報の日付変更線時刻は HH:MM 形式なので、":" で区切る
    let (hour, minute) = match date_line.split_once(':') {
        Some((h, m)) => (
            h.parse::<u32>().unwrap_or(12),
            m.parse::<u32>().unwrap_or(0),
        ),
        None => (12, 0),
    };
    let date_line_time = match NaiveTime::from_hms_opt(hour, minute, 0) {
        Some(value) => value,
        None => return Err(anyhow::anyhow!("")),
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
