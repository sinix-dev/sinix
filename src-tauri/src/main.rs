#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use sinix::udp_server;

mod cmd;

fn main() {
  sinix::init();
  let udpSocketServer = udp_server::UdpSocketServer::new();

  tauri::AppBuilder::new()
    .setup(sinix::tauri_handler)
    .plugin(udpSocketServer)
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;

      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            MyCustomCommand { argument } => {
              //  your command code
              println!("{}", argument);
            }
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
