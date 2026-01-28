use crate::context::Context;
use crate::dto::user_dto;
use crate::service::user_service;

//用户登录
#[tauri::command]
pub fn login(ctx: tauri::State<Context>, body: user_dto::LoginReq) -> user_dto::LoginRes {
  user_service::login(&ctx, &body.username, &body.password)
    .map(|_| user_dto::LoginRes {
      status: 200,
      data: "登录成功".to_string(),
    })
    .unwrap_or_else(|err| user_dto::LoginRes {
      status: 500,
      data: err,
    })
}
