pub mod udp_server;
pub mod channel;
pub mod models;
mod event_handlers;
mod game;

use dirs;
use std::fs;
use std::path::Path;
use std::thread;
use tauri::Webview;
use tokio::runtime;

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
