mod extract_and_copy;
mod rename;

use std::fs;
use std::path::Path;
use serde::Serialize;

use rename::rename;
use extract_and_copy::extract_and_copy;

use crate::constants as sinix_constants;

#[derive(Serialize)]
pub struct Reply {
  pub data: String,
}

/// Create Games Directories
pub fn init(){
  let home_dir = tauri::api::path::home_dir().unwrap();
  let games_dir = Path::new(&home_dir)
    .join(sinix_constants::GAMES_DIR)
    .to_str()
    .unwrap()
    .to_string();
  let data_dir = Path::new(&home_dir)
    .join(sinix_constants::GAMES_DATA_DIR)
    .to_str()
    .unwrap()
    .to_string();

  fs::create_dir_all(&games_dir).unwrap();
  fs::create_dir_all(&data_dir).unwrap();
}

/// Install Sinix App
pub fn install(mut webview: tauri::WebviewMut, msg: Option<String>) {
  let file_name = msg.unwrap();
  extract_and_copy(file_name);

  // TODO: Write some relevant message
  let reply = Reply {
    data: "something else".to_string(),
  };

  tauri::event::emit(
    &mut webview,
    String::from("sinix-install-response"),
    Some(serde_json::to_string(&reply).unwrap()),
  )
  .expect("failed to emit");
}
