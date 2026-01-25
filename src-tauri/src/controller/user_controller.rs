use crate::dto::user_dto;
use crate::service::user_service;

//用户登录
#[tauri::command]
pub fn login(body: user_dto::LoginReq) -> user_dto::LoginRes {
  println!(
    "开始登录 username = {}; password = {}",
    body.username, body.password
  );

  if user_service::login(&body.username, &body.password) {
    user_dto::LoginRes {
      status: 200,
      data: "登录成功".to_string(),
    }
  } else {
    user_dto::LoginRes {
      status: 500,
      data: "账号或密码错误".to_string(),
    }
  }
}
