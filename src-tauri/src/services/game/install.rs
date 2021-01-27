use std::fs;
use std::io;
use std::path::Path;

use zip;
use serde_json;
use serde::Serialize;

use super::config as game_config;

#[derive(Serialize)]
pub struct Reply {
  pub data: String,
}

/// Rename the temp installation directory to the actualy game directory
///
/// In the process of installtion, the game get extracted in a temp directory
/// so that it is never the case where the game is partially installed
fn rename() {
  let home_dir = tauri::api::path::home_dir().unwrap();
  let tmp_dir = Path::new(&home_dir)
    .join(game_config::TEMP_DIR)
    .to_str()
    .unwrap()
    .to_string();

  let manifest_path = Path::new(&home_dir)
    .join(game_config::TEMP_DIR)
    .join(game_config::SINIX_CONFIG_FILENAME)
    .to_str()
    .unwrap()
    .to_string();

  println!("{}", manifest_path);

  let manifest = fs::read_to_string(manifest_path)
    .expect("Unable to read file");

  let manifest: serde_json::Value =
    serde_json::from_str(&manifest).expect("JSON was not well-formatted");

  let game_dir = Path::new(&home_dir)
    .join(game_config::GAMES_DIR)
    .join(manifest["slug"].as_str().unwrap())
    .to_str()
    .unwrap()
    .to_string();

  fs::rename(&tmp_dir, &game_dir).unwrap_or_else(|_err| {
    fs::remove_dir_all(&game_dir).unwrap();
    fs::rename(tmp_dir, game_dir).unwrap();
  });
}

/// Extract and Copy the content to a temperory directory
///
/// The affected temp directory later gets renamed to be the legit game
/// assets directory
fn extract_and_copy(file_name: String) {
  let file = fs::File::open(&file_name).unwrap();
  let mut archive = zip::ZipArchive::new(file).unwrap();

  let home_dir = tauri::api::path::home_dir().unwrap();
  let temp_dir = Path::new(&home_dir)
    .join(game_config::TEMP_DIR)
    .to_str()
    .unwrap()
    .to_string();

  fs::create_dir_all(&temp_dir).unwrap();

  for i in 0..archive.len() {
    let mut file = archive.by_index(i).unwrap();
    let outpath = Path::new(&temp_dir).join(file.name());

    {
      let comment = file.comment();
      if !comment.is_empty() {
        println!("File {} comment: {}", i, comment);
      }
    }

    if (&*file.name()).ends_with("/") {
      println!(
        "File {} extracted to \"{}\"",
        i,
        outpath.as_path().display()
      );

      fs::create_dir_all(&outpath).unwrap();
    } else {
      println!(
        "File {} extracted to \"{}\" ({} bytes)",
        i,
        outpath.as_path().display(),
        file.size()
      );

      if let Some(p) = outpath.parent() {
        if !p.exists() {
          fs::create_dir_all(&p).unwrap();
        }
      }

      let mut outfile = fs::File::create(&outpath).unwrap();
      io::copy(&mut file, &mut outfile).unwrap();
    }

    // Get and Set permissions
    #[cfg(unix)]
    {
      use std::os::unix::fs::PermissionsExt;

      if let Some(mode) = file.unix_mode() {
        fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
      }
    }
  }

  rename();
}

/// Install Sinix App
pub fn install(mut webview: tauri::WebviewMut, msg: Option<String>) {
  let file_name = msg.unwrap();
  extract_and_copy(file_name);

  // TODO: Write some relevant message
  let reply = Reply {
    data: "something else".to_string(),
  };

  tauri::event::emit(
    &mut webview,
    String::from("sinix-install-response"),
    Some(serde_json::to_string(&reply).unwrap()),
  )
  .expect("failed to emit");
}
