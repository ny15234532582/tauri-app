mod context;
mod controller;
mod dao;
mod dto;
mod entity;
mod service;

use tauri::Manager;

// 引入具体 controller
use controller::user_controller;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let db = dao::init_db(app.handle());
      let ctx = context::Context { db };
      app.manage(ctx);
      Ok(())
    })
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![user_controller::login])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
