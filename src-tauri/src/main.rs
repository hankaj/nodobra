#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use polars::prelude::*;
use serde::{ser::SerializeSeq, Serialize, Serializer};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::{self, Manager};

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
        df: DataFrame,
    },
    Multiply {},
    Average {},
}

use Node::*;

fn serialize_columns<S>(df: &DataFrame, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let columns = df.get_column_names();

    let mut seq = serializer.serialize_seq(Some(columns.len()))?;

    for column in columns {
        seq.serialize_element(column)?;
    }

    seq.end()
}

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
        Node::Multiply {}
    }
    fn average() -> Self {
        Node::Average {}
    }
}

fn send_state(app: &tauri::AppHandle, state: &State) {
    let mut result: Option<DataFrame> = None;

    for node in state.nodes.iter() {
        match node {
            LoadData { df } => {
                result = Some(df.clone());
            }
            Multiply {} => {
                if let Some(ref mut df) = result {
                    df.replace_at_idx(0, (&df[0]) * 5.0);
                }
            }
            Average {} => {
                if let Some(ref mut df) = result {
                    *df = df.sum();
                }
            }
        }
    }

    #[derive(Serialize, Clone)]
    struct Data<'a> {
        nodes: &'a State,
        #[serde(serialize_with = "df_to_string")]
        result: &'a Option<DataFrame>,
    }

    let data = Data {
        nodes: state,
        result: &result,
    };

    app.emit_all("show-data", data).unwrap();
}

fn df_to_string<S>(df: &Option<DataFrame>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match df {
        Some(df) => serializer.serialize_str(&df.to_string()),
        None => serializer.serialize_none(),
    }
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
        .invoke_handler(tauri::generate_handler![
            add_loader,
            add_averager,
            add_multiplier
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
