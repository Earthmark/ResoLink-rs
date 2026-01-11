use crate::data_model::Field;
use crate::data_model::primitives::*;
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
    #[serde(rename = "string", alias = "empty")]
    String(Field<Option<String>>),
    #[serde(rename = "Uri")]
    Uri(Field<Option<String>>),
    #[serde(rename = "enum")]
    Enum(Field<Option<String>>),

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
    #[serde(rename = "int?")]
    NullInt(Field<Option<i32>>),
    #[serde(rename = "long")]
    Long(Field<i64>),

    #[serde(rename = "int2")]
    Int2(Field<Int2>),
    #[serde(rename = "int2?")]
    NullInt2(Field<Option<Int2>>),
    #[serde(rename = "int3")]
    Int3(Field<Int3>),
    #[serde(rename = "int4")]
    Int4(Field<Int4>),

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
    #[serde(rename = "colorX?")]
    NullColorX(Field<Option<ColorX>>),

    #[serde(rename = "color32")]
    Color32(Field<Color32>),

    // Vectors and matrices...
    #[serde(rename = "float2")]
    Float2(Field<Float2>),
    #[serde(rename = "float3")]
    Float3(Field<Float3>),
    #[serde(rename = "float3?")]
    NullFloat3(Field<Option<Float3>>),
    #[serde(rename = "float4")]
    Float4(Field<Float4>),

    #[serde(rename = "floatQ")]
    FloatQ(Field<FloatQ>),
    #[serde(rename = "floatQ?")]
    NullFloatQ(Field<Option<FloatQ>>),
    // TODO: Implement more fields.
}

macro_rules! match_id {
    ($target:ident, $($field:ident),+ $(,)?) => {
        match $target {
            $(
                $field(f) => f.id(),
            )+
        }
    };
}

impl super::ID for Member {
    fn id(&self) -> &str {
        use Member::*;
        match_id!(
            self, Reference, SyncList, SyncObject, String, Uri, Enum, Byte, UShort, UInt, ULong,
            SByte, Short, NullInt, Int, NullInt2, Int2, Int3, Int4, Long, Float, Float3, NullFloat3, FloatQ, Float2, Float4,
            NullFloatQ, NullFloat, Double, Bool, NullBool, Color, ColorX, Color32, NullColorX
        )
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
