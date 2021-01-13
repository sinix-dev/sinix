use std::str;
use std::thread;
use std::net::UdpSocket;
use std::sync::mpsc::channel;
use tauri::{plugin::Plugin, Webview};

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
      let socket = match UdpSocket::bind("0.0.0.0:41433") {
        Ok(s) => s,
        Err(e) => panic!("couldn't bind socket: {}", e)
      };

      let mut buf = [0; 2048];

      loop {
        match socket.recv_from(&mut buf) {
          Ok((amt, _src)) => {
            let reply = str::from_utf8(&buf[..amt]).unwrap_or("");
            let reply = crate::models::Reply {
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
    });

    thread::spawn(move || {
      for received in rx {
        println!("Got: {}", received);
        tauri::event::listen(String::from("udp-event"), move |msg| {
          println!("{}", msg.unwrap());
        })
      }
    });
  }
}
