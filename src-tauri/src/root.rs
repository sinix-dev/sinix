use tauri::{plugin::Plugin, Webview};
use crate::channel;
use crate::game;

pub struct SinixRoot;

impl SinixRoot {
  pub fn new() -> Self {
    Self {}
  }
}

fn init(){
  channel::init();
  game::init();
}

impl Plugin for SinixRoot {
  fn created(&self, webview: &mut Webview) {
    let webview = webview.as_mut();

    tauri::event::listen(String::from("sinix-install"), move |msg| {
      game::install(webview.clone(), msg)
    });

    init();
  }
}
