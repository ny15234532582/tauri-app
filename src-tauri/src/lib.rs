mod controller;
mod dao;
mod dto;
mod entity;
mod service;

// 引入具体 controller
use controller::user_controller;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![user_controller::login])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
