use std::str;
use std::net::UdpSocket;
use std::sync::mpsc::Sender;
use crate::config;
use serde::Serialize;

#[derive(Serialize)]
pub struct Reply {
  pub data: String,
}

/// Initiate UDP Server on PORT number `config::UDP_SERVER_PORT`
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
