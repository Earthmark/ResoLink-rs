use crate::data_model::ID;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Empty {
    pub id: String,
}

impl ID for Empty {
    fn id(&self) -> &str {
        &self.id
    }
}
