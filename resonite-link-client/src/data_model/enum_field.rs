use crate::data_model::{ID};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Enum {
    pub id: String,

    pub value: String,
    pub enum_type: String,
}

impl ID for Enum {
    fn id(&self) -> &str {
        &self.id
    }
}

