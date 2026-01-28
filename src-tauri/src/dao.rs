pub mod user_dao;

use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};

pub type Db = Arc<Mutex<Connection>>;

fn db_path(app: &AppHandle) -> PathBuf {
  let dir = app.path().app_data_dir().expect("cannot get app_data_dir");

  std::fs::create_dir_all(&dir).unwrap();

  dir.join("app.db")
}

pub fn init_db(app: &AppHandle) -> Db {
  let path = db_path(app);
  let conn = Connection::open(path).expect("open sqlite failed");

  // 非常重要的 SQLite 配置
  conn
    .execute_batch(
      r#"
      PRAGMA journal_mode = WAL;
      PRAGMA foreign_keys = ON;
      "#,
    )
    .unwrap();

  let db = Arc::new(Mutex::new(conn));

  // 每个表各自初始化
  user_dao::init_table(&db).unwrap();
  user_dao::init_default_user(&db).unwrap();

  db
}
