#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use polars::prelude::*;
use serde::Serialize;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::{self, Manager};
use uuid::Uuid as UUID;

mod serialization;

use serialization::*;

#[derive(Serialize, Debug)]
#[serde(transparent)]
struct State {
    #[serde(serialize_with = "serialize_nodes")]
    nodes: HashMap<UUID, Node>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "data", rename_all = "kebab-case")]
enum Node {
    LoadData {
        #[serde(rename = "columns", serialize_with = "serialize_columns")]
        df: DataFrame,
    },
    Multiply {
        source: Option<UUID>,
    },
    Average {
        source: Option<UUID>,
    },
}

use Node::*;

impl Node {
    fn load_data<P>(file_path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let df = CsvReader::from_path(file_path)
            .unwrap()
            .has_header(true)
            .finish()
            .unwrap();

        Node::LoadData { df }
    }

    fn multiply() -> Self {
        Node::Multiply { source: None }
    }
    fn average() -> Self {
        Node::Average { source: None }
    }
}

fn show_nodes(app: &tauri::AppHandle, state: &State) {
    app.emit_all("show-nodes", state).unwrap();
}

#[tauri::command]
fn add_loader(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle, file_path: String) {
    let mut state = state.lock().unwrap();
    let uuid = UUID::new_v4();
    state.nodes.insert(uuid, Node::load_data(file_path));

    show_nodes(&app, &state);
}

#[tauri::command]
fn add_multiplier(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle) {
    let mut state = state.lock().unwrap();
    let uuid = UUID::new_v4();
    state.nodes.insert(uuid, Node::multiply());

    show_nodes(&app, &state);
}

#[tauri::command]
fn add_averager(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle) {
    let mut state = state.lock().unwrap();
    let uuid = UUID::new_v4();
    state.nodes.insert(uuid, Node::average());

    show_nodes(&app, &state);
}

fn apply_processing(nodes: &HashMap<UUID, Node>, uuid: UUID) -> Option<DataFrame> {
    println!("processing {}", uuid);
    let node = &nodes[&uuid];

    match node {
        LoadData { df } => return Some(df.clone()),
        Multiply { source } => source.and_then(|source| {
            apply_processing(nodes, source).map(|mut df| {
                df.replace_at_idx(0, (&df[0]) * 5).unwrap();
            
                return df;
        })
        }),
        Average { source } => source.and_then(|source| {
            apply_processing(nodes, source).map(|df| df.sum())
        }),
    }
}

#[tauri::command]
fn calculate(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle, uuid: String) {
    let mut state = state.lock().unwrap();
    dbg!(&uuid);

    let result = apply_processing(&state.nodes, UUID::parse_str(&uuid).unwrap());

    app.emit_all("show-result", ResultSerializer { result, meta: uuid }).unwrap();
}

#[tauri::command]
fn connect(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle, source_uuid: String, node_uuid: String) {
    let mut state = state.lock().unwrap();

    let source_uuid = UUID::parse_str(&source_uuid).unwrap();
    let node_uuid = UUID::parse_str(&node_uuid).unwrap();

    match state.nodes.get_mut(&node_uuid).unwrap() {
        LoadData { .. } => {},
        Multiply { ref mut source } => { *source = Some(source_uuid); }
        Average { ref mut source } => { *source = Some(source_uuid); }
    }

    dbg!(&state.nodes);

    show_nodes(&app, &state);
}

fn main() {
    let state = Arc::new(Mutex::new(State { nodes: HashMap::new() }));

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            add_loader,
            add_averager,
            add_multiplier,
            calculate,
            connect,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
