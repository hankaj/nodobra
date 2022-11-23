#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(result_flattening)]

use names::Generator;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    sync::{Arc, Mutex},
};

use tauri::{self, Manager};
use uuid::Uuid as UUID;

use env_logger;
use log;
use serde_json;

mod error;
mod node;
mod serialization;

use error::Error;
use node::*;
use serialization::*;

// TODO Replace all `uuid`s with `node_id`s, because the UUIDs may be used for smth else later.

#[derive(Serialize, Debug)]
struct NodeState {
    name: String,

    #[serde(flatten)]
    settings: NodeSettings,

    #[serde(skip)]
    // results: Option<Result<(DataFrame, u64), Error>>,
    results: Option<Result<(DataFrame, u64), Error>>,
}

#[derive(Serialize, Debug)]
struct State {
    nodes: HashMap<UUID, NodeState>,

    /// Edge map, where the key is the `destination` and the value is the `source`.
    edges: HashMap<UUID, UUID>,
}

impl State {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    fn add_node(&mut self, settings: NodeSettings) -> UUID {
        let uuid = UUID::new_v4();
        let name = Generator::default().next().unwrap();

        self.nodes.insert(
            uuid,
            NodeState {
                settings,
                name,
                results: None,
            },
        );

        uuid
    }

    fn add_edge(&mut self, destination: UUID, source: UUID) {
        self.edges.insert(destination, source);
    }
}

fn emit_state(app: &tauri::AppHandle, state: &State) {
    app.emit_all("update_state", state).unwrap();
}

fn emit_error(app: &tauri::AppHandle, error: Error) {
    log::error!("error: {}", serde_json::to_string_pretty(&error).unwrap());

    app.emit_all("error", error).unwrap();
}

#[tauri::command]
fn add_load_data(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle) {
    log::info!("command: add `load data`");

    let mut state = state.lock().unwrap();

    let uuid = state.add_node(NodeSettings::load_data());

    emit_state(&app, &state);
}

#[tauri::command]
fn add_multiply(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle) {
    log::info!("command: add `multiply`");

    let mut state = state.lock().unwrap();

    let uuid = state.add_node(NodeSettings::multiply());

    emit_state(&app, &state);
}

#[tauri::command]
fn add_sum(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle) {
    log::info!("command: add `sum`");

    let mut state = state.lock().unwrap();

    let uuid = state.add_node(NodeSettings::sum());

    emit_state(&app, &state);
}

#[tauri::command]
fn add_tail(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle) {
    log::info!("command: add `tail`");

    let mut state = state.lock().unwrap();

    let uuid = state.add_node(NodeSettings::tail());

    emit_state(&app, &state);
}

#[tauri::command]
fn add_head(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle) {
    log::info!("command: add `head`");

    let mut state = state.lock().unwrap();

    let uuid = state.add_node(NodeSettings::head());

    emit_state(&app, &state);
}

fn compute_input_hash(settings: &NodeSettings, input_hashes: &[u64]) -> u64 {
    let mut state = DefaultHasher::new();
    settings.hash(&mut state);

    for input_hash in input_hashes {
        input_hash.hash(&mut state);
    }

    state.finish()
}

fn compute_node(
    nodes: &HashMap<UUID, NodeState>,
    edges: &HashMap<UUID, UUID>,
    uuid: UUID,
) -> Result<DataFrame, Error> {
    log::info!("action: computing `{}`", uuid);

    let node = &nodes[&uuid];

    match &node.settings {
        NodeSettings::LoadData(LoadData { path, separator }) => {
            let path = path.as_ref().ok_or(Error::MissingFieldData {
                node_id: uuid,
                field: "path".into(),
            })?;
            let separator = separator.as_ref().ok_or(Error::MissingFieldData {
                node_id: uuid,
                field: "separator".into(),
            })?;
            let df = CsvReader::from_path(path)
                .unwrap()
                .with_delimiter(separator.chars().next().unwrap() as u8)
                .has_header(true)
                .finish()
                .unwrap();

            return Ok(df);
        }
        NodeSettings::Multiply(Multiply { times }) => {
            let input_node_id = edges.get(&uuid).ok_or(Error::MissingFieldData {
                node_id: uuid,
                field: "source".into(),
            })?;
            let input_node = nodes.get(input_node_id).ok_or(Error::InternalError)?;

            let (input_df, _input_hash) = input_node
                .results
                .as_ref()
                .ok_or(Error::MissingResults {
                    node_id: *input_node_id,
                })?
                .as_ref()
                .map_err(|_| Error::MissingResults {
                    node_id: *input_node_id,
                })?;
            let times = times.ok_or(Error::MissingFieldData {
                node_id: uuid,
                field: "times".into(),
            })?; // Missing field `times`.

            let mut input_df = input_df.clone();
            input_df.replace_at_idx(0, (&input_df[0]) * times).unwrap();

            return Ok(input_df);
        }
        NodeSettings::Tail(Tail { row_count }) => {
            let input_node_id = edges.get(&uuid).ok_or(Error::MissingFieldData {
                node_id: uuid,
                field: "source".into(),
            })?;
            let input_node = nodes.get(input_node_id).ok_or(Error::InternalError)?;

            let (input_df, _input_hash) = input_node
                .results
                .as_ref()
                .ok_or(Error::MissingResults {
                    node_id: *input_node_id,
                })?
                .as_ref()
                .map_err(|_| Error::MissingResults {
                    node_id: *input_node_id,
                })?;
            // let row_count = row_count.ok_or(Error::MissingFieldData {
            //     node_id: uuid,
            //     field: "row_count".into(),
            // })?;

            let mut input_df = input_df.clone();
            let output_df = input_df.tail(*row_count);

            return Ok(output_df);
        }
        NodeSettings::Head(Head { row_count }) => {
            let input_node_id = edges.get(&uuid).ok_or(Error::MissingFieldData {
                node_id: uuid,
                field: "source".into(),
            })?;
            let input_node = nodes.get(input_node_id).ok_or(Error::InternalError)?;

            let (input_df, _input_hash) = input_node
                .results
                .as_ref()
                .ok_or(Error::MissingResults {
                    node_id: *input_node_id,
                })?
                .as_ref()
                .map_err(|_| Error::MissingResults {
                    node_id: *input_node_id,
                })?;
            // let row_count = row_count.ok_or(Error::MissingFieldData {
            //     node_id: uuid,
            //     field: "row_count".into(),
            // })?;

            let mut input_df = input_df.clone();
            let output_df = input_df.head(*row_count);

            return Ok(output_df);
        }
        NodeSettings::Sum(Sum {}) => {
            let input_node_id = edges.get(&uuid).ok_or(Error::MissingFieldData {
                node_id: uuid,
                field: "source".into(),
            })?;
            let input_node = nodes.get(input_node_id).ok_or(Error::InternalError)?;

            let (input_df, _input_hash) = input_node
                .results
                .as_ref()
                .ok_or(Error::MissingResults {
                    node_id: *input_node_id,
                })?
                .as_ref()
                .map_err(|_| Error::MissingResults {
                    node_id: *input_node_id,
                })?;

            return Ok(input_df.clone().sum());
        }
    }
}

fn calculate_inner(state: &mut State, node_id: UUID) -> Result<(), Error> {
    let mut execution_queue = vec![];
    let mut layer = vec![node_id];

    loop {
        let next_layer = layer.iter().filter_map(|node_id| state.edges.get(node_id)).collect::<Vec<_>>();
        
        execution_queue.extend(layer.drain(..));
        layer.extend(next_layer);

        if layer.is_empty() {
            break;
        }
    }

    for node_id in execution_queue.into_iter().rev() {
        let new_hash = {
            let mut hasher = DefaultHasher::new();
            
            if let Some(result) = state.edges.get(&node_id).map(|node_id| state.nodes.get(node_id).ok_or(Error::InternalError)) {
                let input_node = result?;
                input_node.settings.hash(&mut hasher);
            }
    
            let node = &state.nodes[&node_id];

            node.settings.hash(&mut hasher);
    
            hasher.finish()
        };

        let node = &state.nodes[&node_id];

        let has_up_to_date_results = match &node.results {
            Some(Ok((_df, prev_hash))) => {
                new_hash == *prev_hash
            },
            _ => false,
        };

        if !has_up_to_date_results {
            log::info!("`{}` does not have up to date results, computing.", node_id);
            let results = compute_node(&state.nodes, &state.edges, node_id);
            let node_state = state.nodes.get_mut(&node_id).ok_or(Error::InternalError)?;
            node_state.results = Some(results.map(|df| (df, new_hash)));
        } else {
            log::info!("`{}` has up to date results, no need to compute.", node_id);
            // Do nothing, the results are fresh.
        }
    }

    Ok(())
}

#[tauri::command]
fn calculate(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle, node_id: UUID) {
    log::info!("command: calculate `{}`", node_id);

    let mut state = state.lock().unwrap();

    let result = calculate_inner(&mut state, node_id)
        .and_then(|()| {
            let (df, _hash) = state
                .nodes
                .get(&node_id)
                .ok_or(Error::InternalError)?
                .results
                .as_ref()
                .ok_or(Error::InternalError)?
                .as_ref()
                .map_err(|err| err.clone())?;

            return Ok(df);
        });

    match result {
        Ok(df) => {
            app.emit_all(
                "show_result",
                json!({
                    "result": df.to_string(),
                    "node_id": node_id,
                }),
            )
            .unwrap();
        }
        Err(err) => {
            app.emit_all("error", json!({ "message": format!("{:?}", err) }))
                .unwrap();
        }
    }
}

#[tauri::command]
fn add_edge(
    state: tauri::State<Arc<Mutex<State>>>,
    app: tauri::AppHandle,
    source: String,
    destination: String,
) {
    log::info!("command: add edge from `{}` to `{}`", source, destination);

    let mut state = state.lock().unwrap();

    let source = UUID::parse_str(&source).unwrap();
    let destination = UUID::parse_str(&destination).unwrap();
    state.add_edge(destination, source);

    emit_state(&app, &state);
}

#[tauri::command]
fn get_nodes(state: tauri::State<Arc<Mutex<State>>>, app: tauri::AppHandle) {
    log::info!("command: get nodes");

    let mut state = state.lock().unwrap();

    emit_state(&app, &state);
}

#[tauri::command]
fn update_node(
    state: tauri::State<Arc<Mutex<State>>>,
    app: tauri::AppHandle,
    node_id: UUID,
    settings: NodeSettings,
) {
    log::info!(
        "command: update node `{}` with:\n{}",
        node_id,
        serde_json::to_string_pretty(&settings).unwrap()
    );

    let mut state = state.lock().unwrap();

    let node_state = state.nodes.get_mut(&node_id).unwrap();

    if node_state.settings.matches_kind(&settings) {
        node_state.settings = settings.clone();

        emit_state(&app, &state);
    } else {
        emit_error(
            &app,
            Error::SettingsUpdateKindMismatch { node_id, settings },
        );
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let state = Arc::new(Mutex::new(State::new()));

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            add_load_data,
            add_sum,
            add_tail,
            add_head,
            add_multiply,
            calculate,
            add_edge,
            get_nodes,
            update_node,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
