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

    #[serde(rename = "byte")]
    Byte(Field<u8>),
    #[serde(rename = "byte?")]
    NullByte(Field<Option<u8>>),
    #[serde(rename = "ushort")]
    UShort(Field<u16>),
    #[serde(rename = "ushort?")]
    NullUShort(Field<Option<u16>>),
    #[serde(rename = "uint")]
    UInt(Field<u32>),
    #[serde(rename = "uint?")]
    NullUInt(Field<Option<u32>>),
    #[serde(rename = "ulong")]
    ULong(Field<u64>),
    #[serde(rename = "ulong?")]
    NullULong(Field<Option<u64>>),

    #[serde(rename = "sbyte")]
    SByte(Field<i8>),
    #[serde(rename = "sbyte?")]
    NullSByte(Field<Option<i8>>),
    #[serde(rename = "short")]
    Short(Field<i16>),
    #[serde(rename = "short?")]
    NullShort(Field<Option<i16>>),
    #[serde(rename = "int")]
    Int(Field<i32>),
    #[serde(rename = "int?")]
    NullInt(Field<Option<i32>>),
    #[serde(rename = "long")]
    Long(Field<i64>),
    #[serde(rename = "long?")]
    NullLong(Field<Option<i64>>),

    #[serde(rename = "float")]
    Float(Field<f32>),
    #[serde(rename = "float?")]
    NullFloat(Field<Option<f32>>),
    #[serde(rename = "double")]
    Double(Field<f64>),
    #[serde(rename = "double?")]
    NullDouble(Field<Option<f64>>),

    // decimal?
    #[serde(rename = "bool")]
    Bool(Field<bool>),
    #[serde(rename = "bool?")]
    NullBool(Field<Option<bool>>),

    // Char(r)?
    #[serde(rename = "string")]
    String(Field<String>),
    #[serde(rename = "string?")]
    NullString(Field<Option<String>>),

    #[serde(rename = "color")]
    Color(Field<Color>),
    #[serde(rename = "color?")]
    NullColor(Field<Option<Color>>),

    #[serde(rename = "colorX")]
    ColorX(Field<ColorX>),
    #[serde(rename = "colorX?")]
    NullColorX(Field<Option<ColorX>>),

    #[serde(rename = "color32")]
    Color32(Field<Color32>),
    #[serde(rename = "color32?")]
    NullColor32(Field<Option<Color32>>),

    // Vectors and matrices...
    #[serde(rename = "float3")]
    Float3(Field<Float3>),
    #[serde(rename = "float3?")]
    NullFloat3(Field<Option<Float3>>),

    #[serde(rename = "floatq")]
    FloatQ(Field<FloatQ>),
    #[serde(rename = "floatq?")]
    NullFloatQ(Field<Option<FloatQ>>),
}

impl super::ID for Member {
    fn id(&self) -> &str {
        match self {
            Member::Byte(byte) => byte.id(),
            Member::NullByte(byte) => byte.id(),
            Member::UShort(short) => short.id(),
            Member::NullUShort(short) => short.id(),
            Member::UInt(uint) => uint.id(),
            Member::NullUInt(uint) => uint.id(),
            _ => todo!("If you run into this, match cases need to be added for member interface code."),
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
