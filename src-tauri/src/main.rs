#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

use sinix::plugins;
use tokio::runtime::Runtime;

fn main() {
  env_logger::init();

  tauri::spawn(|| {
    let mut rt = Runtime::new().unwrap();

    rt.block_on(async {
      println!("server: {:?}", std::thread::current().id());

      sinix::server::serve().await;
    });
  });

  println!("main: {:?}", std::thread::current().id());

  let sinix_root = plugins::SinixRoot::new();
  let game_webview = plugins::GameWebview::new();
  let udp_socket_server = plugins::UdpSocketServer::new();

  tauri::AppBuilder::new()
    .plugin(udp_socket_server)
    .plugin(sinix_root)
    .plugin(game_webview)
    .build()
    .run();
}
