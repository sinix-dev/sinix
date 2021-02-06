use std::fs;
use std::io;
use std::path::Path;

use zip;

use crate::constants as sinix_constants;

/// Extract and Copy the content to a temperory directory
///
/// The affected temp directory later gets renamed to be the legit game
/// assets directory
pub fn extract_and_copy(file_name: String) {
  let file = fs::File::open(&file_name).unwrap();
  let mut archive = zip::ZipArchive::new(file).unwrap();

  let home_dir = tauri::api::path::home_dir().unwrap();
  let temp_dir = Path::new(&home_dir)
    .join(sinix_constants::TEMP_DIR)
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

  super::rename();
}
