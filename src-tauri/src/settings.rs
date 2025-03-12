use std::{fs, path::PathBuf};

use fs_extra::file::write_all;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

const SETTINGS_FILE_NAME: &str = "settings.json";

#[derive(Debug, Deserialize, Serialize, specta::Type)]
pub struct SettingsStruct {
    /// フォルダ監視間隔（秒）
    interval_sec: u8,
    /// 日付が変わったと判断する時刻（HH:MM）
    date_line: String,
    /// 通知モード
    notification_mode: String,
}

impl SettingsStruct {
    pub fn get_interval_sec(&self) -> u8 {
        self.interval_sec
    }
    pub fn get_date_line(&self) -> String {
        self.date_line.clone()
    }
    pub fn get_notification_mode(&self) -> String {
        self.notification_mode.clone()
    }
}

pub fn load_settings_from_file(app: &AppHandle) -> SettingsStruct {
    let default_settings: SettingsStruct = SettingsStruct {
        interval_sec: 60,
        date_line: String::from("12:00"),
        notification_mode: String::from("onSuccess"),
    };

    let settings_file_path = match get_settings_file_path(app) {
        Ok(value) => value,
        Err(_) => return default_settings,
    };
    if settings_file_path.exists() {
        let settings_content = match fs::read_to_string(settings_file_path) {
            Ok(value) => value,
            Err(_) => return default_settings,
        };
        match serde_json::from_str::<SettingsStruct>(&settings_content) {
            Ok(value) => return value,
            Err(_) => return default_settings,
        };
    } else {
        return default_settings;
    }
}

pub fn save_settings_to_file(app: &AppHandle, settings: &SettingsStruct) -> anyhow::Result<()> {
    let settings_file_path = get_settings_file_path(app).unwrap();
    if !settings_file_path.exists() {
        // ファイルが存在しない場合、フォルダも存在しないはずなので
        // まずフォルダを作成する
        let config_dir_path = app.path().app_config_dir()?;
        fs::create_dir(config_dir_path)?;
    }
    let json_string = serde_json::to_string(settings)?;
    Ok(write_all(settings_file_path, &json_string)?)
}

fn get_settings_file_path(app: &AppHandle) -> anyhow::Result<PathBuf> {
    let config_dir_path = app.path().app_config_dir()?;
    Ok(config_dir_path.join(SETTINGS_FILE_NAME))
}
