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
];

fn search_in_steam_dirs(file: &str){
    if let Some(home_dir) = path::home_dir() {
        for candidate in STEAM_DATA_DIRS {
            println!("{:?}", home_dir.join(candidate).canonicalize())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn scan_dirs() {
        super::search_in_steam_dirs("bleh")
    }
}
