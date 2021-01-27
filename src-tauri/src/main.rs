#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use sinix::root;
use sinix::plugins;

mod cmd;

fn main() {
  let sinix_root = root::SinixRoot::new();
  let game_webview = plugins::GameWebview::new();
  let udp_socket_server = plugins::UdpSocketServer::new();

  tauri::AppBuilder::new()
    .plugin(udp_socket_server)
    .plugin(sinix_root)
    .plugin(game_webview)
    .build()
    .run();
}
