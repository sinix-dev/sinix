use super::routes;
use super::constants as sinix_constants;

use warp::Filter;

pub async fn serve() {
  let game_route = routes::game::route();
  let health_route = routes::health::route();
  let channel_route = routes::channel::route();

  let routes = health_route
    .or(channel_route)
    .or(game_route)
    .with(
      warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTION"]),
    );

  warp::serve(routes)
    .run(([0, 0, 0, 0], sinix_constants::SERVER_PORT))
    .await;
}
