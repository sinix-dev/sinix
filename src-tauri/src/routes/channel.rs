use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;

use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter, Rejection, Reply};

mod handler;
mod ws;

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<RwLock<HashMap<String, Client>>>;

#[derive(Debug, Clone)]
pub struct Client {
  pub username: String,
  pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
  let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

  let register = warp::path("register");
  let register_routes = register
    .and(warp::post())
    .and(warp::body::form())
    .and(with_clients(clients.clone()))
    .and_then(handler::register_handler)
    .or(
      register
        .and(warp::delete())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(handler::unregister_handler),
    );

  let publish = warp::path!("publish")
    .and(warp::body::json())
    .and(with_clients(clients.clone()))
    .and_then(handler::publish_handler);

  let ws_route = warp::path("ws")
    .and(warp::ws())
    .and(warp::path::param())
    .and(with_clients(clients))
    .and_then(handler::ws_handler);

  warp::path!("channel")
  .and(
    register_routes
      .or(publish)
      .or(ws_route)
  )
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
  warp::any().map(move || clients.clone())
}
