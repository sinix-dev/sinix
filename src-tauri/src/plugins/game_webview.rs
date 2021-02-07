use tauri::{plugin::Plugin, Webview};
use webview_official::{SizeHint, WebviewBuilder};

use crate::constants as sinix_constants;

mod models {
  use serde::Deserialize;

  #[derive(Deserialize, Debug)]
  pub struct GameWindowMetrics {
    pub width: usize,
    pub height: usize
  }

  #[derive(Deserialize, Debug)]
  pub struct SinixConfig {
    pub name: String,
    pub version: String,
    pub title: String,
    pub slug: String,
    pub window: Option<GameWindowMetrics>
  }
}

#[derive(Default)]
pub struct GameWebview; 

impl GameWebview {
  pub fn new() -> Self {
    Self {}
  }
}

impl Plugin for GameWebview {
  fn created(&self, _webview: &mut Webview) {
    tauri::event::listen(String::from("game-webview"), move |msg| {
      let game_id = msg.unwrap();
      let game_url = format!("http://127.0.0.1:{}/{}", sinix_constants::SERVER_PORT, game_id);
      let index_url = format!("{}/index.html", game_url);
      let config_url = format!("{}/sinix.manifest.json", game_url);

      let response = reqwest::blocking::get(&config_url)
        .unwrap();
      
      let config = match response.json::<models::SinixConfig>() {
        Ok(conf) => conf,
        Err(error) => panic!("Problem parsing response, {:?}", error),
      };

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
        .url(&index_url)
        .build();

      webview.run();
    })
  }
}
