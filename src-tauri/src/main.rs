// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayEvent, SystemTrayMenu};

/// システムトレイインスタンスを生成して返す
fn create_system_tray_menu() -> SystemTray {
    let menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("settings".to_string(), "設定"))
        .add_item(CustomMenuItem::new("quit".to_string(), "終了"));
    SystemTray::new().with_menu(menu)
}

use tauri::Manager;

/// メインウィンドウ（設定画面）を開く
///
/// * `app` - AppHandle
fn show_settings_window(app: &tauri::AppHandle) {
    app.get_window("main").unwrap().show().unwrap();
}

fn main() {
    let tray = create_system_tray_menu();

    tauri::Builder::default()
        // .invoke_handler()
        .system_tray(tray)
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
                    std::process::exit(0);
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
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
