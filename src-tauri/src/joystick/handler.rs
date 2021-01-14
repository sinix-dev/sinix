use std::sync::mpsc::Receiver;

/// [WIP] Takes the signals and transmits to the game running on Sinix
pub fn handle(rx: Receiver<String>){
  for received in rx {
    println!("Got: {}", received);
    tauri::event::listen(String::from("udp-event"), move |msg| {
      println!("{}", msg.unwrap());
    })
  }
}
