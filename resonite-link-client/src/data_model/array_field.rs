use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArrayField<T> {
    pub id: String,
    pub values: Vec<T>,
}

impl<T> ArrayField<T> {
    pub fn new(id: impl Into<String>, values: Vec<T>) -> Self {
        Self {
            id: id.into(),
            values,
        }
    }
}

impl<T> super::ID for ArrayField<T> {
    fn id(&self) -> &str {
        &self.id
    }
}
