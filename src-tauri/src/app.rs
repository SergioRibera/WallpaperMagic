#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub fn launch_app_gui () {
    tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
