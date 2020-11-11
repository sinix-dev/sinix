use actix_files as fs;
use actix_web::{App, HttpServer};
use std::path::Path;

const GAMES_DIR: &str = ".sinix/games";

#[actix_rt::main]
pub async fn serve() -> std::io::Result<()> {
  println!("Serving games...");

  HttpServer::new(|| {
    let home_dir = dirs::home_dir().unwrap();
    let games_dir = Path::new(&home_dir)
      .join(GAMES_DIR)
      .to_str()
      .unwrap()
      .to_string();

    println!("{}", games_dir);

    App::new().service(fs::Files::new("/", &games_dir))
  })
  .bind("127.0.0.1:41432")?
  .run()
  .await
}
