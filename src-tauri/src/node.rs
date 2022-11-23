use std::path::PathBuf;

use polars::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid as UUID;

use super::serialization::serialize_columns;

#[derive(Serialize, Deserialize, Hash, Debug, Clone)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum NodeSettings {
    LoadData(LoadData),
    Multiply(Multiply),
    Sum(Sum),
    Tail(Tail),
    Head(Head),
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct LoadData {
    pub separator: Option<String>,
    pub path: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct Multiply {
    pub times: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct Tail {
    pub row_count: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct Head {
    pub row_count: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub struct Sum {}

impl NodeSettings {
    pub fn load_data() -> Self {
        NodeSettings::LoadData(LoadData {
            path: None,
            separator: Some(",".into()),
        })
    }

    pub fn multiply() -> Self {
        NodeSettings::Multiply(Multiply { times: None })
    }

    pub fn sum() -> Self {
        NodeSettings::Sum(Sum {})
    }

    pub fn tail() -> Self {
        NodeSettings::Tail(Tail { row_count: None })
    }

    pub fn head() -> Self {
        NodeSettings::Head(Head { row_count: None })
    }

    pub fn matches_kind(&self, new_settings: &NodeSettings) -> bool {
        use NodeSettings::*;

        match (self, new_settings) {
            (LoadData { .. }, LoadData { .. })
            | (Multiply { .. }, Multiply { .. })
            | (Tail { .. }, Tail { .. })
            | (Head { .. }, Head { .. })
            | (Sum { .. }, Sum { .. }) => true,
            _ => false,
        }
    }
}
