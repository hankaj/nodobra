#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{self, Manager};
use std::{
  sync::{Arc, Mutex},
  path::PathBuf,
};
use polars::prelude::*;

struct State {
  nodes: Vec<Node>,
}

struct Node {
  df: DataFrame,
}

impl Node {
  fn new<P>(file_path: P) -> Self where P: Into<PathBuf> {
    let df = CsvReader::from_path(file_path)
      .unwrap()
      .has_header(true)
      .finish()
      .unwrap();

    Self {
      df
    }
  }
}

#[tauri::command]
fn load_csv(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle, file_path: String) {
  let mut state = state.lock().unwrap();
  state.nodes.push(Node::new(file_path));

  let data = state.nodes.iter().map(|node| node.df.get_column_names().join(", ")).collect::<Vec<_>>().join("\n");
  app.emit_all("show-data", data).unwrap();
}

fn main() {
  let state = Arc::new(Mutex::new(State { nodes: vec![] }));

  tauri::Builder::default()
    .manage(state)
    .invoke_handler(tauri::generate_handler![load_csv])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
