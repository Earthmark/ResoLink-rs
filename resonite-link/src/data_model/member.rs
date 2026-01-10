use crate::data_model::Field;
use crate::data_model::primitives::{Color, Color32, ColorX, Float3, FloatQ};
use crate::data_model::reference::Reference;
use crate::data_model::sync_list::SyncList;
use crate::data_model::sync_object::SyncObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase", tag = "$type")]
pub enum Member {
    #[serde(rename = "reference")]
    Reference(Reference),

    #[serde(rename = "list")]
    SyncList(SyncList),
    #[serde(rename = "syncObject")]
    SyncObject(SyncObject),

    // Char(r)?
    #[serde(rename = "string")]
    String(Field<Option<String>>),

    #[serde(rename = "byte")]
    Byte(Field<u8>),
    #[serde(rename = "ushort")]
    UShort(Field<u16>),
    #[serde(rename = "uint")]
    UInt(Field<u32>),
    #[serde(rename = "ulong")]
    ULong(Field<u64>),

    #[serde(rename = "sbyte")]
    SByte(Field<i8>),
    #[serde(rename = "short")]
    Short(Field<i16>),
    #[serde(rename = "int")]
    Int(Field<i32>),
    #[serde(rename = "long")]
    Long(Field<i64>),

    #[serde(rename = "float")]
    Float(Field<f32>),
    #[serde(rename = "float?")]
    NullFloat(Field<Option<f32>>),
    #[serde(rename = "double")]
    Double(Field<f64>),

    // decimal?
    #[serde(rename = "bool")]
    Bool(Field<bool>),
    #[serde(rename = "bool?")]
    NullBool(Field<Option<bool>>),

    #[serde(rename = "color")]
    Color(Field<Color>),

    #[serde(rename = "colorX")]
    ColorX(Field<ColorX>),

    #[serde(rename = "color32")]
    Color32(Field<Color32>),

    // Vectors and matrices...
    #[serde(rename = "float3")]
    Float3(Field<Float3>),

    #[serde(rename = "floatq")]
    FloatQ(Field<FloatQ>),

    // TODO: Implement more fields.
}

impl super::ID for Member {
    fn id(&self) -> &str {
        use Member::*;
        match self {
            Reference(f) => f.id(),
            SyncList(f) => f.id(),
            SyncObject(f) => f.id(),
            String(f) => f.id(),
            Byte(f) => f.id(),
            UShort(f) => f.id(),
            UInt(f) => f.id(),
            ULong(f) => f.id(),
            SByte(f) => f.id(),
            Short(f) => f.id(),
            Int(f) => f.id(),
            Long(f) => f.id(),
            Float(f) => f.id(),
            NullFloat(f) => f.id(),
            Double(f) => f.id(),
            Bool(f) => f.id(),
            NullBool(f) => f.id(),
            Color(f) => f.id(),
            ColorX(f) => f.id(),
            Color32(f) => f.id(),
            Float3(f) => f.id(),
            FloatQ(f) => f.id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_bi_eq_json;
    use serde_json::json;

    #[test]
    fn reference() {
        assert_bi_eq_json(
            Member::Reference(Reference::new("Taco", "Target", "Type")),
            json!({
                "$type": "reference",
                "id": "Taco",
                "targetId": "Target",
                "targetType": "Type",
            }),
        );
    }

    #[test]
    fn byte() {
        assert_bi_eq_json(
            Member::Byte(Field::new("Taco", 2)),
            json!({
                "$type": "byte",
                "id": "Taco",
                "value": 2,
            }),
        );
    }

    #[test]
    fn nullable_float() {
        assert_bi_eq_json(
            Member::NullFloat(Field::new("Taco", Some(3f32))),
            json!({
                "$type": "float?",
                "id": "Taco",
                "value": 3.0,
            }),
        );
    }

    #[test]
    fn nullable_float_null() {
        assert_bi_eq_json(
            Member::NullFloat(Field::new("Taco", None)),
            json!({
                "$type": "float?",
                "id": "Taco",
                "value": null,
            }),
        );
    }

    #[test]
    fn float3() {
        assert_bi_eq_json(
            Member::Float3(Field::new("Taco", Float3::default())),
            json!({
                "$type": "float3",
                "id": "Taco",
                "value": {
                    "x": 0.0,
                    "y": 0.0,
                    "z": 0.0,
                },
            }),
        );
    }
}
