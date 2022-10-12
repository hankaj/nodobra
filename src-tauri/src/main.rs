#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{self, Manager};
use std::{
  sync::{Arc, Mutex},
  path::PathBuf,
  collections::HashMap,
};
use polars::prelude::*;
use serde::{Serialize, Serializer, ser::SerializeSeq};

#[derive(Serialize, Debug)]
#[serde(transparent)]
struct State {
  nodes: Vec<Node>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
enum Node {
  LoadData {
    #[serde(rename = "columns", serialize_with = "serialize_columns")]
    df: DataFrame
  },
  Multiply {},
  Average {},
}

fn serialize_columns<S>(df: &DataFrame, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
  let columns = df.get_column_names();

  let mut seq = serializer.serialize_seq(Some(columns.len()))?;

  for column in columns {
      seq.serialize_element(column)?;
  }

  seq.end()
}

impl Node {
  fn load_data<P>(file_path: P) -> Self where P: Into<PathBuf> {
    let df = CsvReader::from_path(file_path)
      .unwrap()
      .has_header(true)
      .finish()
      .unwrap();

    Node::LoadData { df }
  }

  fn multiply() -> Self { Node::Multiply {} }
  fn average() -> Self { Node::Average {} }
}

fn send_state(app: &tauri::AppHandle, state: &State) {
  app.emit_all("show-data", state).unwrap();
}

#[tauri::command]
fn add_loader(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle, file_path: String) {
  let mut state = state.lock().unwrap();
  state.nodes.push(Node::load_data(file_path));

  send_state(&app, &state);
}

#[tauri::command]
fn add_multiplier(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle) {
  let mut state = state.lock().unwrap();
  state.nodes.push(Node::multiply());

  send_state(&app, &state);
}

#[tauri::command]
fn add_averager(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle) {
  let mut state = state.lock().unwrap();
  state.nodes.push(Node::average());

  send_state(&app, &state);
}

fn main() {
  let state = Arc::new(Mutex::new(State { nodes: vec![] }));

  tauri::Builder::default()
    .manage(state)
    .invoke_handler(tauri::generate_handler![add_loader, add_averager, add_multiplier])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
