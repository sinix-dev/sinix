use std::path::Path;

use actix_files as fs;
use actix_web::{App, HttpServer};

use super::config as game_config;
use crate::config;

/// Static server to serve games directory
#[actix_rt::main]
pub async fn serve() -> std::io::Result<()> {
  println!("Serving games...");

  HttpServer::new(|| {
    let home_dir = tauri::api::path::home_dir().unwrap();
    let games_dir = Path::new(&home_dir)
      .join(game_config::GAMES_DIR)
      .to_str()
      .unwrap()
      .to_string();

    println!("voila: {:?}", std::thread::current().id());

    App::new().service(fs::Files::new("/", &games_dir))
  })
  .bind(format!("127.0.0.1:{}", config::GAME_SERVER_PORT))?
  .run()
  .await
}
