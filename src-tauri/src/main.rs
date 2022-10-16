#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use polars::prelude::*;
use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tauri::{self, Manager};
use uuid::Uuid as UUID;

use env_logger;
use log;
use serde_json;

mod node;
mod serialization;

use node::*;
use serialization::*;

#[derive(Serialize, Debug)]
#[serde(transparent)]
struct State {
    nodes: HashMap<UUID, Node>,
}

impl State {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: Node) {
        self.nodes.insert(UUID::new_v4(), node);
    }
}

fn show_nodes(app: &tauri::AppHandle, state: &State) {
    app.emit_all("show_nodes", state).unwrap();
}

#[tauri::command]
fn add_load_data(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle) {
    log::info!("command: add `load data`");
    let mut state = state.lock().unwrap();
    state.add_node(Node::load_data());

    show_nodes(&app, &state);
}

#[tauri::command]
fn add_multiply(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle) {
    log::info!("command: add `multiply`");
    let mut state = state.lock().unwrap();
    state.add_node(Node::multiply());

    show_nodes(&app, &state);
}

#[tauri::command]
fn add_sum(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle) {
    log::info!("command: add `sum`");
    let mut state = state.lock().unwrap();
    state.add_node(Node::sum());

    show_nodes(&app, &state);
}

fn apply_processing(nodes: &HashMap<UUID, Node>, uuid: UUID) -> Option<DataFrame> {
    println!("processing {}", uuid);
    let node = &nodes[&uuid];

    match node {
        Node::LoadData(LoadData { df, .. }) => return df.clone(),
        Node::Multiply(Multiply { source, times, .. }) => source
            .and_then(|source| times.map(|times| (source, times)))
            .and_then(|(source, times)| {
                apply_processing(nodes, source).map(|mut df| {
                    df.replace_at_idx(0, (&df[0]) * times).unwrap();

                    return df;
                })
            }),
        Node::Sum(Sum { source, .. }) => {
            source.and_then(|source| apply_processing(nodes, source).map(|df| df.sum()))
        }
    }
}

#[tauri::command]
fn calculate(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle, uuid: String) {
    log::info!("command: calculate");
    let mut state = state.lock().unwrap();

    let result = apply_processing(&state.nodes, UUID::parse_str(&uuid).unwrap());

    app.emit_all("show_result", ResultSerializer { result, meta: uuid })
        .unwrap();
}

#[tauri::command]
fn connect(
    state: tauri::State<Arc<Mutex<State>>>,
    app: tauri::AppHandle,
    source_uuid: String,
    node_uuid: String,
) {
    log::info!("command: connect {} to {}", source_uuid, node_uuid);

    let mut state = state.lock().unwrap();

    let source_uuid = UUID::parse_str(&source_uuid).unwrap();
    let node_uuid = UUID::parse_str(&node_uuid).unwrap();

    match state.nodes.get_mut(&node_uuid).unwrap() {
        Node::LoadData(LoadData { .. }) => {}
        Node::Multiply(Multiply { ref mut source, .. }) => {
            *source = Some(source_uuid);
        }
        Node::Sum(Sum { ref mut source, .. }) => {
            *source = Some(source_uuid);
        }
    }

    show_nodes(&app, &state);
}

#[tauri::command]
fn get_nodes(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle) {
    log::info!("command: get nodes");

    let mut state = state.lock().unwrap();

    show_nodes(&app, &state);
}

#[tauri::command]
fn update_node(
    state: tauri::State<Arc<Mutex<State>>>,
    app: tauri::AppHandle,
    patch: NodePatchWrapper,
) {
    log::info!(
        "command: update node with {}",
        serde_json::to_string_pretty(&patch).unwrap()
    );

    let NodePatchWrapper { uuid, inner: patch } = patch;
    let mut state = state.lock().unwrap();

    let node = (*state.nodes.get(&uuid).unwrap()).clone();

    match patch.patch_node(node) {
        Ok(node) => {
            state.nodes.insert(uuid, node);
        }
        Err(e) => log::error!("{}", e),
    }

    show_nodes(&app, &state);
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let state = Arc::new(Mutex::new(State::new()));

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            add_load_data,
            add_sum,
            add_multiply,
            calculate,
            connect,
            get_nodes,
            update_node,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
