#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod game;

fn main() {
  pub fn init() {
    thread::spawn(move || {
      runtime::Runtime::new().unwrap().block_on(async {
        channel::init().await;
      });
    });
  
    let home_dir = dirs::home_dir().unwrap();
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
      game::serve().unwrap();
    });
  }
  
  tauri::AppBuilder::new()
    .setup(sinix::tauri_handler)
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;

      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            // definitions for your custom commands from Cmd here
            MyCustomCommand { argument } => {
              //  your command code
              println!("{}", argument);
            }
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
