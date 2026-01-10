macro_rules! impl_bi_into {
    ($dm_type:ty, $grpc_type:ty, [$($field:ident $($opt:ident)?),+ $(,)?]) => {
        impl From<$dm_type> for $grpc_type {
            fn from(src: $dm_type) -> Self {
                Self {
                    $(
                        $field: to_dm_field!(src.$field$($opt)?),
                    )+
                }
            }
        }

        impl From<$grpc_type> for $dm_type {
            fn from(src: $grpc_type) -> Self {
                Self {
                    $(
                        $field: to_grpc_field!(src.$field$($opt)?),
                    )+
                }
            }
        }
    };
}

macro_rules! to_dm_field {
    ($src:ident.$field:ident opt) => {
        Some($src.$field.into())
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
        $src.$field.unwrap_or_default().into()
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

macro_rules! impl_field {
    ($dm_type:ty,$grpc_type:ty) => {
        impl_bi_into!(data_model::Field<$dm_type>, $grpc_type, [id, value]);
    };
}

macro_rules! impl_opt_field {
    ($dm_type:ty,$grpc_type:ty) => {
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
pub(crate) use impl_opt_field;
