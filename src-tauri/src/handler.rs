use crate::{ws, Client, Clients, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reply::json, ws::Message, Reply};

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
  username: String,
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
  hue: usize,
}

#[derive(Deserialize, Debug)]
pub struct Event {
  event_type: String,
  username: String,
  payload: String,
}

pub async fn publish_handler(body: Event, clients: Clients) -> Result<impl Reply> {
  let clients = clients.read().await;

  let username = String::from(&body.username);

  if !clients.contains_key(&username) {
    println!("No such user");
    return Ok(StatusCode::BAD_REQUEST);
  }

  let client = &clients[&username];

  if let Some(sender) = &client.sender {
    let _ = sender.send(Ok(Message::text(body.username.clone())));
  }

  Ok(StatusCode::OK)
}

pub async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl Reply> {
  let username = body.username;
  let hue: usize = rand::thread_rng().gen_range(0, 360);

  register_client(username.clone(), hue, clients).await;

  Ok(json(&RegisterResponse { hue }))
}

async fn register_client(username: String, hue: usize, clients: Clients) {
  let username_copy = username.clone();

  clients.write().await.insert(
    username_copy,
    Client {
      username,
      hue,
      sender: None,
    },
  );
}

pub async fn unregister_handler(id: String, clients: Clients) -> Result<impl Reply> {
  clients.write().await.remove(&id);
  Ok(StatusCode::OK)
}

pub async fn ws_handler(
  ws: warp::ws::Ws,
  username: String,
  clients: Clients,
) -> Result<impl Reply> {
  let client = clients.read().await.get(&username).cloned();

  match client {
    Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, username, clients, c))),
    None => Err(warp::reject::not_found()),
  }
}

pub async fn health_handler() -> Result<impl Reply> {
  Ok(StatusCode::OK)
}
