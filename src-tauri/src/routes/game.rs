use warp::{Filter, Reply};

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
  warp::path("game").and(warp::fs::dir("/Users/sanket143/.sinix/games"))
}
