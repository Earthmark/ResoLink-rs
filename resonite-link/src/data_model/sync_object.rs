use crate::data_model::{ID, Member};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SyncObject {
    pub id: String,

    pub members: HashMap<String, Member>,
}

impl ID for SyncObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_model::Field;
    use crate::test_utils::assert_bi_eq_json;
    use serde_json::json;

    #[test]
    fn serialize() {
        assert_bi_eq_json(
            SyncObject {
                id: "Taco".into(),
                members: HashMap::from([(
                    "Field1".into(),
                    Member::Bool(Field::new("Potato", false)),
                )]),
            },
            json!({
                "id": "Taco",
                "members": {
                    "Field1": {
                        "$type": "bool",
                        "id": "Potato",
                        "value": false
                    }
                },
            }),
        );
    }
}
