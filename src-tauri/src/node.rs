use std::path::PathBuf;

use names::Generator;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid as UUID;

use super::serialization::serialize_columns;

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum Node {
    LoadData(LoadData),
    Multiply(Multiply),
    Average(Average),
}

pub mod nodes {
    use super::*;

    #[derive(Serialize, Debug, Clone)]
    pub struct LoadData {
        pub name: String,
        #[serde(rename = "columns", serialize_with = "serialize_columns")]
        pub df: DataFrame,
    }

    #[derive(Serialize, Debug, Clone)]
    pub struct Multiply {
        pub name: String,
        pub times: Option<i64>,
        pub source: Option<UUID>,
    }

    #[derive(Serialize, Debug, Clone)]
    pub struct Average {
        pub name: String,
        pub source: Option<UUID>,
    }
}

use self::nodes::*;

#[derive(Deserialize, Debug)]
pub struct NodePatchWrapper {
    pub uuid: UUID,
    #[serde(flatten)]
    pub inner: NodePatch,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum NodePatch {
    Multiply { times: Option<i64> },
}

impl NodePatch {
    pub fn patch_node(&self, node: Node) -> Result<Node, String> {
        let node = match (self, node) {
            (NodePatch::Multiply { times, .. }, Node::Multiply(node)) => {
                let times = times.or(node.times);

                Node::Multiply(Multiply { times, ..node })
            }
            _ => return Err("bad :(".to_string()),
        };

        Ok(node)
    }
}

impl Node {
    pub fn load_data<P>(file_path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let df = CsvReader::from_path(file_path)
            .unwrap()
            .has_header(true)
            .finish()
            .unwrap();
        let name = Generator::default().next().unwrap();

        Node::LoadData(LoadData { name, df })
    }

    pub fn multiply() -> Self {
        let name = Generator::default().next().unwrap();

        Node::Multiply(Multiply {
            name,
            times: None,
            source: None,
        })
    }

    pub fn average() -> Self {
        let name = Generator::default().next().unwrap();

        Node::Average(Average { name, source: None })
    }
}
