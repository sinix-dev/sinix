pub mod channel;
pub mod game;
pub mod health;

use warp::Rejection;

type Result<T> = std::result::Result<T, Rejection>;
