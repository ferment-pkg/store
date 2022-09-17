#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri::Manager;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
