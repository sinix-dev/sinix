use tauri::{plugin::Plugin, Webview};
use webview_official::{SizeHint, WebviewBuilder};

use crate::constants as sinix_constants;
use std::fs;
use std::path::Path;

mod models {
  use serde::{Deserialize, Serialize};

  #[derive(Serialize, Deserialize, Debug)]
  pub struct GameWindowMetrics {
    pub width: usize,
    pub height: usize,
  }

  #[derive(Serialize, Deserialize, Debug)]
  pub struct SinixConfig {
    pub name: String,
    pub version: String,
    pub title: String,
    pub slug: String,
    pub window: Option<GameWindowMetrics>,
  }
}

#[derive(Default)]
pub struct Game;

impl Game {
  pub fn new() -> Self {
    Self {}
  }
}

impl Plugin for Game {
  fn created(&self, webview: &mut Webview) {
    let mut webview = webview.as_mut();
    tauri::event::listen(String::from("game-list"), move |_| {
      let home_dir = tauri::api::path::home_dir().unwrap();
      let games_dir = Path::new(&home_dir)
        .join(sinix_constants::GAMES_DIR)
        .to_str()
        .unwrap()
        .to_string();

      let paths = fs::read_dir(games_dir).unwrap();

      for path in paths {
        let path = path.unwrap().path();
        let game_slug = path.file_name().unwrap().to_str().unwrap();

        let config = get_game_config(&game_slug);
        println!("Config: {:?}", config);

        tauri::event::emit(&mut webview, String::from("game-item"), Some(config))
          .expect("failed to emit");
      }
    });

    tauri::event::listen(String::from("game-webview"), move |msg| {
      let game_id = msg.unwrap();
      let game_url = format!(
        "http://127.0.0.1:{}/game/{}/index.html",
        sinix_constants::SERVER_PORT,
        game_id
      );

      let config = get_game_config(&game_id);

      println!("{:?}", config);

      let game_window = match config.window {
        Some(window) => window,
        None => models::GameWindowMetrics {
          width: 1024,
          height: 768,
        },
      };

      let mut webview = WebviewBuilder::new()
        .debug(true)
        .title(config.title.as_str())
        .width(game_window.width)
        .height(game_window.height)
        .resize(SizeHint::FIXED)
        .url(&game_url)
        .build();

      webview.run();
    })
  }
}

fn get_game_config(slug: &str) -> models::SinixConfig {
  let home_dir = tauri::api::path::home_dir().unwrap();
  let config_file = Path::new(&home_dir)
    .join(sinix_constants::GAMES_DIR)
    .join(slug)
    .join("sinix.manifest.json");

  let config = fs::read_to_string(config_file).expect("Something went wrong reading the file");

  let config: models::SinixConfig = serde_json::from_str(config.as_str()).unwrap();

  return config;
}
