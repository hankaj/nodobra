use serde::Serialize;
use uuid::Uuid as UUID;

use crate::node::NodeSettings;

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "kind", content = "data", rename_all = "snake_case")]
pub enum Error {
    SettingsUpdateKindMismatch {
        node_id: UUID,
        settings: NodeSettings,
    },
    InvalidUUID {
        uuid: String,
    },
    MissingResults {
        node_id: UUID,
    },
    MissingFieldData {
        node_id: UUID,
        field: String,
    },
    InternalError,
}
