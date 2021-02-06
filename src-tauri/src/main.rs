#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

use sinix::plugins;

#[tokio::main]
async fn main() {
  env_logger::init();
  tokio::spawn(async {
    sinix::server::serve().await;
  });

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
