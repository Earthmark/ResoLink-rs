mod component;
mod enum_field;
mod field;
mod floats;
mod member;
mod primitives;
mod reference;
mod slot;
mod sync_list;
mod sync_object;
mod empty;
mod array_field;

pub use component::Component;
pub use enum_field::Enum;
pub use field::Field;
pub use floats::{F32, F64};
pub use member::Member;
pub use primitives::*;
pub use reference::Reference;
pub use slot::Slot;
pub use sync_list::SyncList;
pub use empty::Empty;
pub use sync_object::SyncObject;
pub use array_field::ArrayField;

pub trait ID {
    fn id(&self) -> &str;
}

pub trait Worker: ID {
    fn is_reference_only(&self) -> bool;
}
