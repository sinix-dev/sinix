use std::fs;
use tauri::api::path;

static STEAM_DATA_DIRS: &'static [&str] = &[
    ".steam",
    ".local/share/steam",
    ".local/share/Steam",
    ".steam/steam",
    ".var/app/com.valvesoftware.Steam/data/steam",
    ".steam/debian-installation",
    "/usr/share/steam",
    "/usr/local/share/steam",
    "Library/Application Support/Steam"
];

fn search_in_steam_dirs(file: &str) -> Option<String> {
    if let Some(home_dir) = path::home_dir() {
        for candidate in STEAM_DATA_DIRS {
            if let Ok(file_path) = home_dir.join(candidate).join(file).canonicalize() {
                return Some(String::from(file_path.to_str().unwrap()));
            }
        }
    }

    return None;
}

fn get_steam_user() -> super::util::SteamUser {
    let filename = search_in_steam_dirs("config/loginusers.vdf").unwrap();
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    crate::services::steam::util::steam_vdf_parser(&contents)
}

#[cfg(test)]
mod tests {
    #[test]
    fn scan_dirs() {
        let steam_user = super::get_steam_user();
        println!("{:?}", steam_user);
    }
}
