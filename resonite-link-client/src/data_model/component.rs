use crate::data_model::{ID, Member, Worker};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub id: String,
    pub is_reference_only: bool,
    pub component_type: String,
    #[serde(deserialize_with = "crate::serde_helpers::null_to_default")]
    pub members: HashMap<String, Member>,
}

impl Worker for Component {
    fn is_reference_only(&self) -> bool {
        self.is_reference_only
    }
}

impl ID for Component {
    fn id(&self) -> &str {
        &self.id
    }
}
