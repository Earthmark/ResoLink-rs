use crate::data_model::{ID, Member};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SyncList {
    pub id: String,

    #[serde(deserialize_with = "crate::serde_helpers::null_to_default")]
    pub elements: Vec<Member>,
}

impl ID for SyncList {
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
            SyncList {
                id: "Taco".into(),
                elements: vec![Member::Bool(Field::new("Potato", false))],
            },
            json!({
                "id": "Taco",
                "elements": [{
                    "$type": "bool",
                    "id": "Potato",
                    "value": false
                }],
            }),
        );
    }
}
