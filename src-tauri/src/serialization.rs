use polars::prelude::*;
use serde::{ser::SerializeSeq, Serialize, Serializer};

pub fn serialize_columns<S>(df: &Option<DataFrame>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(df) = df {
        let columns = df.get_column_names();

        let mut seq = serializer.serialize_seq(Some(columns.len()))?;

        for column in columns {
            seq.serialize_element(column)?;
        }

        seq.end()
    } else {
        serializer.serialize_none()
    }
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
