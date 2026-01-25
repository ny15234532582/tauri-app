use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginReq {
  pub username: String,
  pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginRes {
  pub status: u32,
  pub data: String,
}
