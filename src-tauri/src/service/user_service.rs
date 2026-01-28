use rusqlite::Error::QueryReturnedNoRows;

use crate::context::Context;
use crate::dao::user_dao;

//用户登录
pub fn login(ctx: &Context, username: &str, password: &str) -> Result<(), String> {
  match user_dao::find_user(&ctx.db, username) {
    Ok(user) => {
      if user.password == password {
        Ok(())
      } else {
        Err("账号或密码错误".to_string())
      }
    }
    Err(QueryReturnedNoRows) => Err("未查找到当前用户".to_string()),
    Err(_) => Err("数据库操作失败".to_string()),
  }
}
