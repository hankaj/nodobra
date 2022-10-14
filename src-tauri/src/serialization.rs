use polars::prelude::*;
use super::Node;
use serde::{ser::SerializeSeq, Serialize, Serializer};
use uuid::Uuid as UUID;
use std::collections::HashMap;


pub(super) fn serialize_nodes<S>(nodes: &HashMap<UUID, Node>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct NodeSerializer<'a> {
        uuid: &'a UUID,
        #[serde(flatten)]
        node: &'a Node,
    }

    let mut seq = serializer.serialize_seq(Some(nodes.len()))?;

    for (uuid, node) in nodes.iter() {
        seq.serialize_element(&NodeSerializer { uuid, node })?;
    }

    seq.end()
}


pub fn serialize_columns<S>(df: &DataFrame, serializer: S) -> Result<S::Ok, S::Error>
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


#[derive(Serialize, Clone)]
pub struct ResultSerializer {
    #[serde(serialize_with = "serialize_df")]
    pub result: Option<DataFrame>,
    pub meta: String,
}


pub fn serialize_df<S>(df: &Option<DataFrame>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match df {
        Some(df) => serializer.serialize_str(&df.to_string()),
        None => serializer.serialize_none(),
    }
}
