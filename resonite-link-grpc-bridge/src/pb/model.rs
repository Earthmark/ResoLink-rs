use super::convert_utils::*;
use super::*;
use ::resonite_link_client::data_model;
use log::warn;

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

impl_field!(Option<String>, FieldString);
// TODO: Implement smaller integer types, they need conversion attempts instead of full conversions.
// impl_field!(u8, FieldByte);
// impl_field!(u16, FieldUShort);
impl_field!(u32, FieldUInt);
impl_field!(u64, FieldULong);
// impl_field!(i8, FieldSByte);
// impl_field!(i16, FieldShort);
impl_field!(i32, FieldInt);
impl_field!(i64, FieldLong);

impl_field!(f32, FieldFloat);
impl_field!(Option<f32>, FieldNullFloat);
impl_field!(f64, FieldDouble);

impl_field!(bool, FieldBool);
impl_field!(Option<bool>, FieldNullBool);

impl_opt_field!(data_model::Float3, FieldFloat3);
impl_opt_field!(data_model::FloatQ, FieldFloatQ);
impl_opt_field!(data_model::Color, FieldColor);
impl_opt_field!(data_model::ColorX, FieldColorX);
// TODO: Requires downscaling from 32 to 8 bits.
// impl_opt_field!(data_model::Float32, FieldColor32);

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
                Byte(_f) => todo!(), // PB::Byte(f.into()),
                UShort(_f) => todo!(), // PB::Ushort(f.into()),
                UInt(f) => PB::Uint(f.into()),
                ULong(f) => PB::Ulong(f.into()),
                SByte(_f) => todo!(), // PB::Sbyte(f.into()),
                Short(_f) => todo!(), // PB::Short(f.into()),
                Int(f) => PB::Int(f.into()),
                Long(f) => PB::Long(f.into()),
                Float(f) => PB::Float(f.into()),
                NullFloat(f) => PB::NullFloat(f.into()),
                Double(f) => PB::Double(f.into()),
                Bool(f) => PB::Bool(f.into()),
                NullBool(f) => PB::NullBool(f.into()),
                Color(f) => PB::Color(f.into()),
                ColorX(f) => PB::ColorX(f.into()),
                Color32(_f) => todo!(), // PB::Color32(f.into()),
                Float3(f) => PB::Float3(f.into()),
                FloatQ(f) => PB::FloatQ(f.into()),
            }),
        }
    }
}

impl From<Member> for data_model::Member {
    fn from(value: Member) -> Self {
        if let Some(kind) = value.member_kind {
            use super::member::MemberKind::*;
            type DM = data_model::Member;
            match kind {
                Reference(f) => DM::Reference(f.into()),
                SyncList(f) => DM::SyncList(f.into()),
                SyncObject(f) => DM::SyncObject(f.into()),
                String(f) => DM::String(f.into()),
                Byte(_f) => todo!(), // DM::Byte(f.into()),
                Ushort(_f) => todo!(), // DM::UShort(f.into()),
                Uint(f) => DM::UInt(f.into()),
                Ulong(f) => DM::ULong(f.into()),
                Sbyte(_f) => todo!(), // DM::SByte(f.into()),
                Short(_f) => todo!(), // DM::Short(f.into()),
                Int(f) => DM::Int(f.into()),
                Long(f) => DM::Long(f.into()),
                Float(f) => DM::Float(f.into()),
                NullFloat(f) => DM::NullFloat(f.into()),
                Double(f) => DM::Double(f.into()),
                Bool(f) => DM::Bool(f.into()),
                NullBool(f) => DM::NullBool(f.into()),
                Float3(f) => DM::Float3(f.into()),
                FloatQ(f) => DM::FloatQ(f.into()),
                Color(f) => DM::Color(f.into()),
                ColorX(f) => DM::ColorX(f.into()),
                Color32(_f) => todo!(), // DM::Color32(f.into()),
            }
        } else {
            warn!(
                "GRPC Member was not set, defaulting to a bool. This will cause rejected messages."
            );
            Self::Bool(data_model::Field::new("ERR", false))
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
        let output: Slot = mid.into();
        assert_eq!(output, make_test_slot());
    }
}
