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
