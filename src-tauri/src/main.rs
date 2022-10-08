#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
pub mod manager;
use std::sync::Mutex;

use manager::Manager as PackageManager;

use tauri::{Manager, State};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn install_package(manager: State<'_, Mutex<PackageManager>>, package: String) -> String {
    let mut manager = manager.lock().unwrap();
    manager.install_package(package);
    "Installed".to_string()
}
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            Ok(())
        })
        .manage(PackageManager::new())
        .invoke_handler(tauri::generate_handler![install_package])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
