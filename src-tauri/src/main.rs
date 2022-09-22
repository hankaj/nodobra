#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri;
use std::{
  sync::{Arc, Mutex},
};

struct State {
  x: i32,
}

#[tauri::command]
fn my_custom_command(state: tauri::State<Arc<Mutex<State>>>, invoke_message: String) {
  let mut state = state.lock().unwrap();
  println!("I was invoked from JS, with this message: {}. x = {}", invoke_message, state.x);
  state.x += 1;
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
