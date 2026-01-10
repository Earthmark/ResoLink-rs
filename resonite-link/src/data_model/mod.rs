mod component;
mod slot;
mod field;
mod primitives;
mod member;
mod reference;
mod sync_list;
mod sync_object;

pub use component::Component;
pub use slot::Slot;
pub use field::Field;
pub use member::Member;
pub use reference::Reference;
pub use sync_object::SyncObject;
pub use sync_list::SyncList;

pub trait ID {
    fn id(&self) -> &str;
}

pub trait Worker : ID {
    fn is_reference_only(&self) -> bool;
}
