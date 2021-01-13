#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use sinix::joystick;
use sinix::root;

mod cmd;

fn main() {
  let udp_socket_server = joystick::UdpSocketServer::new();
  let sinix_root = root::SinixRoot::new();

  println!("main: {:?}", std::thread::current().id());
  tauri::AppBuilder::new()
    .plugin(udp_socket_server)
    .plugin(sinix_root)
    .build()
    .run();
}
