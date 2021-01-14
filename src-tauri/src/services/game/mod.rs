mod install;
mod config;
mod serve;

pub use install::install;
pub use serve::serve;

use std::fs;
use std::thread;
use std::path::Path;

/// Initialize Gaming Environment
pub fn init(){
  let home_dir = tauri::api::path::home_dir().unwrap();
  let games_dir = Path::new(&home_dir)
    .join(".sinix/games")
    .to_str()
    .unwrap()
    .to_string();
  let data_dir = Path::new(&home_dir)
    .join(".sinix/data")
    .to_str()
    .unwrap()
    .to_string();

  fs::create_dir_all(&games_dir).unwrap();
  fs::create_dir_all(&data_dir).unwrap();

  thread::spawn(move || {
    println!("game: {:?}", std::thread::current().id());
    serve().unwrap();
  });
}
