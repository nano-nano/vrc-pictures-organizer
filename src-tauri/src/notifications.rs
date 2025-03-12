use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

use crate::settings::load_settings_from_file;

pub fn show_files_count_notification(app_handle: &AppHandle, files_count: &i8) {
    let settings = load_settings_from_file(app_handle);

    if settings.get_notification_mode() == "none" {
        return;
    }
    if settings.get_notification_mode() == "onSuccess" && *files_count == 0 {
        return;
    }

    let _ = app_handle
        .notification()
        .builder()
        .body(&format!("{}件のファイルを処理しました", files_count))
        .show();
}
