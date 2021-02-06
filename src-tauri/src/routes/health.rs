use warp::{Filter, Reply};
use warp::http::StatusCode;
use super::Result;

async fn health_handler() -> Result<impl Reply> {
  Ok(StatusCode::OK)
}

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
  warp::path!("health").and_then(health_handler)
}
