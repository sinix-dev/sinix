mod udp;
mod handler;

use std::thread;
use std::sync::mpsc::channel;
use tauri::{plugin::Plugin, Webview};

/// [Tauri Plugin] Spawns a UDP server to receive joystick signals
///
/// Receives payload in following format
/// ```json
/// {
///   "type": "STICK1" /* this could be STICK2 or BUTTON */
///   "payload": {
///     "STICK1": {
///       "x": 1,      /* X OFFSET */
///       "y": 1.2     /* X OFFSET */
///       "val": "UP"  /* Direction */
///     }
///   }
/// }
/// ```
///
/// and notifies the game running on Sinix through events
pub struct UdpSocketServer; 

impl UdpSocketServer {
  pub fn new() -> Self {
    Self {}
  }
}

impl Plugin for UdpSocketServer {
  fn created(&self, _webview: &mut Webview) {
    let (tx, rx) = channel();

    thread::spawn(move || {
      println!("udp_serve: {:?}", std::thread::current().id());
      udp::serve(tx);
    });

    thread::spawn(move || {
      println!("udp_handle: {:?}", std::thread::current().id());
      handler::handle(rx);
    });
  }
}
