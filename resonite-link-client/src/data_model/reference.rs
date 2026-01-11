use crate::data_model::ID;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    pub id: String,

    pub target_id: Option<String>,
    pub target_type: String,
}

impl ID for Reference {
    fn id(&self) -> &str {
        &self.id
    }
}

impl Reference {
    pub fn new(
        id: impl Into<String>,
        target_id: impl Into<String>,
        target_type: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            target_id: Some(target_id.into()),
            target_type: target_type.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_bi_eq_json;
    use serde_json::json;

    #[test]
    fn serializer() {
        assert_bi_eq_json(
            Reference::new("Taco", "Target", "Type"),
            json!({
                "id": "Taco",
                "targetId": "Target",
                "targetType": "Type",
            }),
        );
    }
}
