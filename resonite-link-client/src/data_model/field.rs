use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Field<T> {
    pub id: String,
    pub value: T,
}

impl<T> Field<T> {
    pub fn new(id: impl Into<String>, value: T) -> Self {
        Self {
            id: id.into(),
            value,
        }
    }
}

impl<T> super::ID for Field<T> {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_model::primitives::Float3;
    use crate::test_utils::assert_bi_eq_json;
    use serde_json::json;

    #[test]
    fn number_value() {
        assert_bi_eq_json(
            Field::new("Taco", 2),
            json!({
                "id": "Taco",
                "value": 2,
            }),
        );
    }

    #[test]
    fn bool_value() {
        assert_bi_eq_json(
            Field::new("Taco", true),
            json!({
                "id": "Taco",
                "value": true,
            }),
        );
    }

    #[test]
    fn null_value() {
        assert_bi_eq_json(
            Field::<Option<i32>>::new("Taco", None),
            json!({
                "id": "Taco",
                "value": null,
            }),
        );
    }

    #[test]
    fn null_but_not_value() {
        assert_bi_eq_json(
            Field::<Option<i32>>::new("Taco", Some(4)),
            json!({
                "id": "Taco",
                "value": 4,
            }),
        );
    }

    #[test]
    fn pos_value() {
        assert_bi_eq_json(
            Field::new("Taco", Float3::default()),
            json!({
                "id": "Taco",
                "value": { "x": 0f32, "y": 0f32, "z": 0f32},
            }),
        );
    }
}
