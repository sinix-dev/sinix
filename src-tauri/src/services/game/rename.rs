use std::fs;
use std::path::Path;

use crate::constants as sinix_constants;

/// Rename the temp installation directory to the actualy game directory
///
/// In the process of installtion, the game get extracted in a temp directory
/// so that it is never the case where the game is partially installed
pub fn rename() {
  let home_dir = tauri::api::path::home_dir().unwrap();
  let tmp_dir = Path::new(&home_dir)
    .join(sinix_constants::TEMP_DIR)
    .to_str()
    .unwrap()
    .to_string();

  let manifest_path = Path::new(&home_dir)
    .join(sinix_constants::TEMP_DIR)
    .join(sinix_constants::SINIX_CONFIG_FILENAME)
    .to_str()
    .unwrap()
    .to_string();

  println!("{}", manifest_path);

  let manifest = fs::read_to_string(manifest_path)
    .expect("Unable to read file");

  let manifest: serde_json::Value =
    serde_json::from_str(&manifest).expect("JSON was not well-formatted");

  let game_dir = Path::new(&home_dir)
    .join(sinix_constants::GAMES_DIR)
    .join(manifest["slug"].as_str().unwrap())
    .to_str()
    .unwrap()
    .to_string();

  fs::rename(&tmp_dir, &game_dir).unwrap_or_else(|_err| {
    fs::remove_dir_all(&game_dir).unwrap();
    fs::rename(tmp_dir, game_dir).unwrap();
  });
}
