use crate::{Client, Clients};
use futures::{FutureExt, StreamExt};
use serde::Deserialize;
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

#[derive(Deserialize, Debug)]
pub struct Event {
  event_type: String,
  payload: String,
}

pub async fn client_connection(ws: WebSocket, id: String, clients: Clients, mut client: Client) {
  let (client_ws_sender, mut client_ws_rcv) = ws.split();
  let (client_sender, client_rcv) = mpsc::unbounded_channel();

  tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
    if let Err(e) = result {
      eprintln!("error sending websocket msg: {}", e);
    }
  }));

  client.sender = Some(client_sender);
  clients.write().await.insert(id.clone(), client);

  println!("{} connected", id);

  while let Some(result) = client_ws_rcv.next().await {
    let msg = match result {
      Ok(msg) => msg,
      Err(e) => {
        eprintln!("error receiving ws message for id: {}): {}", id.clone(), e);
        break;
      }
    };
    client_msg(&id, msg, &clients).await;
  }

  clients.write().await.remove(&id);
  println!("{} disconnected", id);
}

async fn client_msg(id: &str, msg: Message, clients: &Clients) {
  println!("received message from {}: {}", id, msg.to_str().unwrap());

  let message = match msg.to_str() {
    Ok(v) => v,
    Err(_) => return,
  };

  let clients = clients.read().await;

  let client = &clients["sinix"];

  if message == "ping" || message == "ping\n" {
    return;
  }

  if let Some(sender) = &client.sender {
    let _ = sender.send(Ok(Message::text(message)));
  }
}
