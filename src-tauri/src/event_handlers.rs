use crate::models::Reply;
use dirs;
use serde_json;
use std::fs;
use std::io;
use std::path::Path;
use zip;

const GAMES_DIR: &str = ".sinix/games";
const TEMP_DIR: &str = ".sinix/games/.tmp_game";

fn rename() {
  let home_dir = dirs::home_dir().unwrap();
  let tmp_dir = Path::new(&home_dir)
    .join(TEMP_DIR)
    .to_str()
    .unwrap()
    .to_string();

  let manifest_path = Path::new(&home_dir)
    .join(TEMP_DIR)
    .join("sinix.manifest.json")
    .to_str()
    .unwrap()
    .to_string();

  println!("{}", manifest_path);

  let manifest = fs::read_to_string(manifest_path).expect("Unable to read file");

  let manifest: serde_json::Value =
    serde_json::from_str(&manifest).expect("JSON was not well-formatted");

  let game_dir = Path::new(&home_dir)
    .join(GAMES_DIR)
    .join(manifest["slug"].as_str().unwrap())
    .to_str()
    .unwrap()
    .to_string();

  fs::rename(&tmp_dir, &game_dir).unwrap_or_else(|_err| {
    fs::remove_dir_all(&game_dir).unwrap();
    fs::rename(tmp_dir, game_dir).unwrap();
  });
}

fn extract_and_copy(file_name: String) {
  let file = fs::File::open(&file_name).unwrap();
  let mut archive = zip::ZipArchive::new(file).unwrap();

  let home_dir = dirs::home_dir().unwrap();
  let temp_dir = Path::new(&home_dir)
    .join(TEMP_DIR)
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

pub fn sinix_install(mut webview: tauri::WebviewMut, msg: Option<String>) {
  // Install a new app
  let file_name = msg.unwrap();
  extract_and_copy(file_name);

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
