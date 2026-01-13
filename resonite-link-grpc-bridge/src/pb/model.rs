use super::convert_utils::*;
use super::*;
use ::resonite_link_client::data_model;
use ::resonite_link_client::data_model::{F32, F64};
use std::convert::Infallible;
use std::num::TryFromIntError;
use thiserror::Error;
use tonic::{Code, Status};

#[derive(Error, Debug)]
pub enum ResoPbMapError {
    #[error("GRPC value for member was not set, and can not be forwarded")]
    UnsetOneOfType,
    #[error(transparent)]
    IntegerOverflow(#[from] TryFromIntError),
}

impl From<ResoPbMapError> for Status {
    fn from(error: ResoPbMapError) -> Self {
        Self::new(Code::InvalidArgument, error.to_string())
    }
}

impl From<Infallible> for ResoPbMapError {
    fn from(value: Infallible) -> Self {
        panic!("attempted to convert into ResoPbMapError {:?}", value)
    }
}

impl_bi_into!(
    data_model::Slot,
    Slot,
    [
        id,
        is_reference_only,
        parent opt,
        name opt,
        tag opt,
        position opt,
        rotation opt,
        scale opt,
        is_active opt,
        is_persistent opt,
        order_offset opt,
        components vec,
        children vec,
    ]
);

impl_bi_into!(
    data_model::Component,
    Component,
    [
        id,
        is_reference_only,
        component_type,
        members map,
    ]
);

impl_bi_into!(
    data_model::Reference,
    Reference,
    [id, target_id, target_type]
);

impl_bi_into!(
    data_model::SyncList,
    SyncList,
    [
        id,
        elements vec
    ]
);

impl_bi_into!(
    data_model::SyncObject,
    SyncObject,
    [
        id,
        members map
    ]
);

impl_bi_into!(data_model::Enum, Enum, [id, value, enum_type,]);

impl_bi_into!(data_model::Empty, Empty, [id,]);

impl_bi_into!(
    data_model::ArrayField<data_model::Float3>,
    FieldFloat3Vec,
    [
        id,
        values vec,
    ]
);

impl_bi_into!(
    data_model::ArrayField<data_model::FloatQ>,
    FieldFloatQVec,
    [
        id,
        values vec,
    ]
);

impl_field!(Option<String>, FieldString);
impl_field!(Option<String>, FieldUri);
impl_field!(Option<String>, FieldEnum);
// TODO: Implement smaller integer types, they need conversion attempts instead of full conversions.
impl_field!(u8, FieldByte);
impl_field!(u16, FieldUShort);
impl_field!(u32, FieldUInt);
impl_field!(u64, FieldULong);
impl_field!(i8, FieldSByte);
impl_field!(i16, FieldShort);
impl_field!(i32, FieldInt);
impl_field!(Option<i32>, FieldNullInt);
impl_field!(i64, FieldLong);

impl_field!(F32, FieldFloat);
impl_field!(Option<F32>, FieldNullFloat, opt);
impl_field!(F64, FieldDouble);

impl_field!(bool, FieldBool);
impl_field!(Option<bool>, FieldNullBool);

impl_field!(data_model::Int2, FieldInt2, opt);
impl_field!(Option<data_model::Int2>, FieldInt2, opt);
impl_field!(data_model::Int3, FieldInt3, opt);
impl_field!(data_model::Int4, FieldInt4, opt);
impl_field!(data_model::Float2, FieldFloat2, opt);
impl_field!(data_model::Float3, FieldFloat3, opt);
impl_field!(Option<data_model::Float3>, FieldFloat3, opt);
impl_field!(data_model::Float4, FieldFloat4, opt);
impl_field!(data_model::FloatQ, FieldFloatQ, opt);
impl_field!(Option<data_model::FloatQ>, FieldFloatQ, opt);
impl_field!(data_model::Color, FieldColor, opt);
impl_field!(data_model::ColorX, FieldColorX, opt);
impl_field!(Option<data_model::ColorX>, FieldColorX, opt);
impl_field!(data_model::Color32, FieldColor32, opt);

impl From<data_model::Member> for Member {
    fn from(value: data_model::Member) -> Self {
        use data_model::Member::*;
        type PB = member::MemberKind;
        Member {
            member_kind: Some(match value {
                Reference(f) => PB::Reference(f.into()),
                SyncList(f) => PB::SyncList(f.into()),
                SyncObject(f) => PB::SyncObject(f.into()),
                String(f) => PB::String(f.into()),
                Uri(f) => PB::Uri(f.into()),
                Enum(f) => PB::Enum(f.into()),
                Empty(f) => PB::Empty(f.into()),
                Byte(f) => PB::Byte(f.into()),
                UShort(f) => PB::Ushort(f.into()),
                UInt(f) => PB::Uint(f.into()),
                ULong(f) => PB::Ulong(f.into()),
                SByte(f) => PB::Sbyte(f.into()),
                Short(f) => PB::Short(f.into()),
                Int(f) => PB::Int(f.into()),
                NullInt(f) => PB::NullInt(f.into()),
                Long(f) => PB::Long(f.into()),
                Float(f) => PB::Float(f.into()),
                NullFloat(f) => PB::NullFloat(f.into()),
                Double(f) => PB::Double(f.into()),
                Bool(f) => PB::Bool(f.into()),
                NullBool(f) => PB::NullBool(f.into()),
                Color(f) => PB::Color(f.into()),
                ColorX(f) => PB::ColorX(f.into()),
                NullColorX(f) => PB::NullColorX(f.into()),
                Color32(f) => PB::Color32(f.into()),
                Float3(f) => PB::Float3(f.into()),
                NullFloat3(f) => PB::NullFloat3(f.into()),
                Float3Vec(f) => PB::Float3Vec(f.into()),
                FloatQ(f) => PB::FloatQ(f.into()),
                FloatQVec(f) => PB::FloatQVec(f.into()),
                NullFloatQ(f) => PB::NullFloatQ(f.into()),
                Int2(f) => PB::Int2(f.into()),
                NullInt2(f) => PB::NullInt2(f.into()),
                Int3(f) => PB::Int3(f.into()),
                Int4(f) => PB::Int4(f.into()),
                Float2(f) => PB::Float2(f.into()),
                Float4(f) => PB::Float4(f.into()),
            }),
        }
    }
}

impl TryFrom<Member> for data_model::Member {
    type Error = ResoPbMapError;
    fn try_from(value: Member) -> Result<Self, ResoPbMapError> {
        if let Some(kind) = value.member_kind {
            use super::member::MemberKind::*;
            type DM = data_model::Member;
            Ok(match kind {
                Reference(f) => DM::Reference(f.try_into()?),
                SyncList(f) => DM::SyncList(f.try_into()?),
                SyncObject(f) => DM::SyncObject(f.try_into()?),
                String(f) => DM::String(f.try_into()?),
                Uri(f) => DM::Uri(f.try_into()?),
                Enum(f) => DM::Enum(f.try_into()?),
                Empty(f) => DM::Empty(f.try_into()?),
                Byte(f) => DM::Byte(f.try_into()?),
                Ushort(f) => DM::UShort(f.try_into()?),
                Uint(f) => DM::UInt(f.try_into()?),
                Ulong(f) => DM::ULong(f.try_into()?),
                Sbyte(f) => DM::SByte(f.try_into()?),
                Short(f) => DM::Short(f.try_into()?),
                Int(f) => DM::Int(f.try_into()?),
                NullInt(f) => DM::NullInt(f.try_into()?),
                Int2(f) => DM::Int2(f.try_into()?),
                NullInt2(f) => DM::NullInt2(f.try_into()?),
                Int3(f) => DM::Int3(f.try_into()?),
                Int4(f) => DM::Int4(f.try_into()?),
                Long(f) => DM::Long(f.try_into()?),
                Float(f) => DM::Float(f.try_into()?),
                NullFloat(f) => DM::NullFloat(f.try_into()?),
                Double(f) => DM::Double(f.try_into()?),
                Bool(f) => DM::Bool(f.try_into()?),
                NullBool(f) => DM::NullBool(f.try_into()?),
                Float2(f) => DM::Float2(f.try_into()?),
                Float3(f) => DM::Float3(f.try_into()?),
                NullFloat3(f) => DM::NullFloat3(f.try_into()?),
                Float3Vec(f) => DM::Float3Vec(f.try_into()?),
                Float4(f) => DM::Float4(f.try_into()?),
                FloatQ(f) => DM::FloatQ(f.try_into()?),
                NullFloatQ(f) => DM::NullFloatQ(f.try_into()?),
                FloatQVec(f) => DM::FloatQVec(f.try_into()?),
                Color(f) => DM::Color(f.try_into()?),
                ColorX(f) => DM::ColorX(f.try_into()?),
                NullColorX(f) => DM::NullColorX(f.try_into()?),
                Color32(f) => DM::Color32(f.try_into()?),
            })
        } else {
            Err(ResoPbMapError::UnsetOneOfType)
        }
    }
}

#[cfg(test)]
mod tests {
    use ::resonite_link_client::data_model::*;

    fn make_test_slot() -> Slot {
        Slot {
            id: "taco".into(),
            is_reference_only: false,
            parent: Reference::new("id", "Target", "Target_type"),
            name: Field::new("n", Some("Name".into())),
            tag: Field::new("t", None),
            position: Field::new(
                "pos",
                Float3 {
                    x: 1.,
                    y: 2.,
                    z: 3.,
                },
            ),
            rotation: Field::new(
                "rot",
                FloatQ {
                    x: 1.,
                    y: 2.,
                    z: 3.,
                    w: 4.,
                },
            ),
            scale: Field::new(
                "scale",
                Float3 {
                    x: 1.,
                    y: 2.,
                    z: 3.,
                },
            ),
            is_active: Field::new("is_active", true),
            is_persistent: Field::new("is_persistent", true),
            order_offset: Field::new("order", 20),
            components: vec![],
            children: vec![],
        }
    }

    #[test]
    fn test_bidirectional() {
        let src = make_test_slot();
        let mid: super::Slot = src.into();
        let output: Slot = mid.try_into().unwrap();
        assert_eq!(output, make_test_slot());
    }
}
