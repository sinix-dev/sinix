use tauri::{plugin::Plugin, Webview};
use webview_official::{SizeHint, WebviewBuilder};

pub struct GameWebview; 

impl GameWebview {
  pub fn new() -> Self {
    Self {}
  }
}

impl Plugin for GameWebview {
  fn created(&self, webview: &mut Webview) {
    tauri::event::listen(String::from("game-webview"), move |msg| {
      println!("{}", msg.unwrap());

      let mut webview = WebviewBuilder::new()
        .debug(true)
        .title("TEST")
        .width(1024)
        .height(768)
        .resize(SizeHint::NONE)
        .url("http://127.0.0.1:41432/svelte-app-v1.0.0/index.html")
        .build();

      webview.run();
    })
  }
}
