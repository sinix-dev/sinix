use std::str;
use std::thread;
use std::net::UdpSocket;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use serde::Serialize;
use tauri::{plugin::Plugin, Webview};

use crate::config;

#[derive(Serialize)]
pub struct Reply {
  pub data: String,
}

/// [Tauri Plugin] Spawns a UDP server to receive joystick signals
///
/// Receives payload in following format
/// ```json
/// {
///   "username": "sanket143",
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
      serve(tx);
    });

    thread::spawn(move || {
      println!("udp_handle: {:?}", std::thread::current().id());
      handle(rx);
    });
  }
}

/// [WIP] Takes the signals and transmits to the game running on Sinix
pub fn handle(rx: Receiver<String>){
  for received in rx {
    println!("Got: {}", received);
    tauri::event::listen(String::from("udp-event"), move |msg| {
      println!("{}", msg.unwrap());
    })
  }
}

pub fn serve(tx: Sender<String>){
  let addr = format!("0.0.0.0:{}", config::UDP_SERVER_PORT);
  let socket = match UdpSocket::bind(addr) {
    Ok(s) => s,
    Err(e) => panic!("couldn't bind socket: {}", e)
  };

  let mut buf = [0; 2048];

  loop {
    match socket.recv_from(&mut buf) {
      Ok((amt, _src)) => {
        let reply = str::from_utf8(&buf[..amt]).unwrap_or("");
        let reply = Reply {
          data: String::from(reply)
        };

        tx.send(serde_json::to_string(&reply).unwrap())
          .map_err(|err| println!("{:?}", err))
          .ok();
      },
      Err(e) => {
        println!("couldn't recieve a datagram: {}", e);
      }
    }
  }
}
