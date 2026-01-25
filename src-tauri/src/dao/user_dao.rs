use crate::entity::user_entity;

//查找当前用户
pub fn find_user(username: &str) -> user_entity::User {
  user_entity::User {
    id: 1,
    username: "admin".to_string(),
    password: "admin123".to_string(),
  }
}
