macro_rules! impl_bi_into {
    ($dm_type:ty, $grpc_type:path, [$($field:ident $($opt:ident)?),+ $(,)?]) => {
/*        impl ResoPBType<$grpc_type> for $dm_type {
            fn forward(self) -> $grpc_type {
                $grpc_type {
                    $(
                        $field: to_dm_field!(self.$field$($opt)?),
                    )+
                }
            }
            fn reverse(other: $grpc_type) -> Result<Self, super::model::ResoPbMapError> {
                Ok(Self {
                    $(
                        $field: to_grpc_field!(other.$field$($opt)?),
                    )+
                })
            }
        }
*/
        impl From<$dm_type> for $grpc_type {
            fn from(src: $dm_type) -> Self {
                Self {
                    $(
                        $field: to_dm_field!(src.$field$($opt)?),
                    )+
                }
            }
        }

        impl TryFrom<$grpc_type> for $dm_type {
            type Error = super::model::ResoPbMapError;
            fn try_from(src: $grpc_type) -> Result<Self, super::model::ResoPbMapError> {
                Ok(Self {
                    $(
                        $field: to_grpc_field!(src.$field$($opt)?),
                    )+
                })
            }
        }
    };
}

macro_rules! to_dm_field {
    ($src:ident.$field:ident opt) => {
        Some($src.$field.into())
    };
    ($src:ident.$field:ident into) => {
        $src.$field.map(Into::into).into()
    };
    ($src:ident.$field:ident vec) => {
        $src.$field.into_iter().map(Into::into).collect()
    };
    ($src:ident.$field:ident map) => {
        $src.$field
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect()
    };
    ($src:ident.$field:ident) => {
        $src.$field.into()
    };
}

macro_rules! to_grpc_field {
    ($src:ident.$field:ident opt) => {
        $src.$field.unwrap_or_default().try_into()?
    };
    ($src:ident.$field:ident into) => {
        $src.$field.map(TryInto::try_into).transpose()?
    };
    ($src:ident.$field:ident vec) => {
        $src.$field.into_iter().map(TryInto::try_into).collect::<Result<_, _>>()?
    };
    ($src:ident.$field:ident map) => {
        $src.$field
            .into_iter()
            .map(|(k, v)| v.try_into().map(|v| (k, v)))
            .collect::<Result<_, _>>()?
    };
    ($src:ident.$field:ident) => {
        $src.$field.try_into()?
    };
}

macro_rules! impl_field {
    ($dm_type:ty, $grpc_type:ty) => {
        impl_bi_into!(data_model::Field<$dm_type>, $grpc_type, [id, value]);
    };
    (Option<$dm_type:ty>, $grpc_type:ty, opt) => {
        impl_bi_into!(data_model::Field<Option<$dm_type>>, $grpc_type, [id, value into]);
    };
    ($dm_type:ty, $grpc_type:ty, opt) => {
        impl_bi_into!(data_model::Field<$dm_type>, $grpc_type, [id, value opt]);
    };
}

pub(crate) use impl_bi_into;
// These are used in the parent macro
#[allow(unused_imports)]
pub(crate) use to_dm_field;
#[allow(unused_imports)]
pub(crate) use to_grpc_field;
pub(crate) use impl_field;
