pub mod game;
pub mod health;
pub mod channel;

use warp::Rejection;

type Result<T> = std::result::Result<T, Rejection>;