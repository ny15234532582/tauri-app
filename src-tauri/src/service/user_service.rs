use crate::dao::user_dao;

//用户登录
pub fn login(username: &str, password: &str) -> bool {
  //查找当前的用户
  let user = user_dao::find_user(username);
  //进行账号密码比对
  user.username == username && user.password == password
}
