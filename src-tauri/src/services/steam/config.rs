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

#[cfg(test)]
mod tests {
    #[test]
    fn scan_dirs() {
        super::search_in_steam_dirs("config/loginusers.vdf");
    }
}
