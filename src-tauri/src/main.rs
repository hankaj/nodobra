#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{self, Manager};
use std::{
  sync::{Arc, Mutex},
  fs::read_to_string,
};

struct State {
  x: i32,
}

#[tauri::command]
fn my_custom_command(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle, file_path: String) {
  let mut state = state.lock().unwrap();

  let data = state.x.to_string() + "  " + &read_to_string(file_path).unwrap();
  state.x += 1;

  app.emit_all("show-data", data).unwrap();
}

fn main() {
  println!("asdf");

  let state = Arc::new(Mutex::new(State { x: 1 }));

  tauri::Builder::default()
    .manage(state)
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
