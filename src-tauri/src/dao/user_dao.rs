use crate::dao::Db;
use crate::entity::user_entity;
use rusqlite::{params, Result};

// 新建数据表
pub fn init_table(db: &Db) -> Result<()> {
  let conn = db.lock().unwrap();
  conn.execute(
    r#"
    CREATE TABLE IF NOT EXISTS users (
      id         INTEGER PRIMARY KEY AUTOINCREMENT,
      name       TEXT NOT NULL,
      username  TEXT NOT NULL UNIQUE,
      password  TEXT NOT NULL
    )
    "#,
    [],
  )?;
  Ok(())
}

// 默认用户
pub fn init_default_user(db: &Db) -> Result<()> {
  let conn = db.lock().unwrap();
  conn.execute(
    r#"
    INSERT OR IGNORE INTO users (name, username, password)
    VALUES ('管理员', 'admin', 'admin123')
    "#,
    [],
  )?;
  Ok(())
}

//查找当前用户
pub fn find_user(db: &Db, username: &str) -> Result<user_entity::User> {
  let conn = db.lock().unwrap();

  let mut stmt =
    conn.prepare("SELECT id, name, username, password FROM users WHERE username = ?1")?;
  let user = stmt.query_row(params![username], |row| {
    Ok(user_entity::User {
      id: row.get(0)?,
      name: row.get(1)?,
      username: row.get(2)?,
      password: row.get(3)?,
    })
  })?;

  Ok(user)
}
