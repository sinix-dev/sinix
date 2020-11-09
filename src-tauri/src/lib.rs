pub mod channel;
mod db;
mod event_handlers;
mod game;
mod models;

use dirs;
use std::fs;
use std::path::Path;
use std::thread;
use tauri::Webview;
use tokio::{runtime, spawn};

pub fn init() {
  thread::spawn(move || {
    runtime::Runtime::new().unwrap().block_on(async {
      channel::init().await;
    });
  });

  let home_dir = dirs::home_dir().unwrap();
  let games_dir = Path::new(&home_dir)
    .join(".sinix/games")
    .to_str()
    .unwrap()
    .to_string();
  let data_dir = Path::new(&home_dir)
    .join(".sinix/data")
    .to_str()
    .unwrap()
    .to_string();

  fs::create_dir_all(&games_dir).unwrap();
  fs::create_dir_all(&data_dir).unwrap();

  thread::spawn(move || {
    game::serve().unwrap();
  });
}

pub fn tauri_handler(webview: &mut Webview, _source: String) {
  // handler function for tauri app setup callback

  let webview = webview.as_mut();

  tauri::event::listen(String::from("sinix-install"), move |msg| {
    event_handlers::sinix_install(webview.clone(), msg)
  });
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let mut db_instance = super::db::init(String::from("sanket143.db"));
    db_instance.set("name", &"sanket").unwrap();
  }
}
