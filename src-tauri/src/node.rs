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
    Sum(Sum),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum NodePatch {
    LoadData(LoadDataPatch),
    Multiply(MultiplyPatch),
    Sum(SumPatch),
}

impl NodePatch {
    pub fn patch_node(self, node: Node) -> Result<Node, String> {
        let node = match (self, node) {
            (NodePatch::Multiply(patch), Node::Multiply(node)) => {
                let times = patch.times.or(node.times);
                let name = patch.name.unwrap_or(node.name);
                let source = patch.source.or(node.source);

                Node::Multiply(Multiply {
                    times,
                    name,
                    source,
                })
            }
            (NodePatch::LoadData(patch), Node::LoadData(node)) => {
                let df = patch
                    .path
                    .map(|path| {
                        let df = CsvReader::from_path(path)
                            .unwrap()
                            .has_header(true)
                            .finish()
                            .unwrap();

                        return df;
                    })
                    .or(node.df);
                let name = patch.name.unwrap_or(node.name);

                Node::LoadData(LoadData { name, df })
            }
            _ => return Err("bad :(".to_string()),
        };

        Ok(node)
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct LoadData {
    pub name: String,
    #[serde(rename = "columns", serialize_with = "serialize_columns")]
    pub df: Option<DataFrame>,
}

#[derive(Serialize, Deserialize)]
struct LoadDataPatch {
    name: Option<String>,
    path: Option<PathBuf>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Multiply {
    pub name: String,
    pub times: Option<i64>,
    pub source: Option<UUID>,
}

#[derive(Serialize, Deserialize)]
struct MultiplyPatch {
    name: Option<String>,
    times: Option<i64>,
    source: Option<UUID>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Sum {
    pub name: String,
    pub source: Option<UUID>,
}

#[derive(Serialize, Deserialize)]
struct SumPatch {
    name: Option<String>,
    source: Option<UUID>,
}

#[derive(Serialize, Deserialize)]
pub struct NodePatchWrapper {
    pub uuid: UUID,
    #[serde(flatten)]
    pub inner: NodePatch,
}

impl Node {
    pub fn load_data() -> Self {
        let name = Generator::default().next().unwrap();

        Node::LoadData(LoadData { name, df: None })
    }

    pub fn multiply() -> Self {
        let name = Generator::default().next().unwrap();

        Node::Multiply(Multiply {
            name,
            times: None,
            source: None,
        })
    }

    pub fn sum() -> Self {
        let name = Generator::default().next().unwrap();

        Node::Sum(Sum { name, source: None })
    }
}
