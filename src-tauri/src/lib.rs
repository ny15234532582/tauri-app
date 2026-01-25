use serde::Serialize;

#[derive(Serialize)]
struct Response {
  status: u32,
  data: String,
}

//用户登录
#[tauri::command]
fn login(username: String, password: String) -> Response {
  println!("开始登录 username = {username}; password = {password}");
  if username == "admin" && password == "admin123" {
    Response{ status: 200, data: "登录成功".to_string() }
  } else {
    Response{ status: 500, data: "账号或密码错误".to_string() }
  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![login])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
