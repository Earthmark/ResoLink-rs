use super::{Component, Field, ID, Worker, Reference};
use crate::data_model::primitives::{Float3, FloatQ};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
    pub id: String,
    pub is_reference_only: bool,

    pub parent: Reference,

    pub name: Field<Option<String>>,
    pub tag: Field<Option<String>>,

    pub position: Field<Float3>,
    pub rotation: Field<FloatQ>,
    pub scale: Field<Float3>,

    pub is_active: Field<bool>,
    pub is_persistent: Field<bool>,

    pub order_offset: Field<i64>,

    /// All the components that belong to this slot.
    #[serde(deserialize_with = "crate::serde_helpers::null_to_default")]
    pub components: Vec<Component>,

    /// All the children that this slot has
    #[serde(deserialize_with = "crate::serde_helpers::null_to_default")]
    pub children: Vec<Slot>,
}

impl Slot {
    pub fn root_slot_id() -> &'static str {
        "Root"
    }

    pub fn is_root_slot(&self) -> bool {
        Some(Self::root_slot_id()) == self.name.value.as_deref().into()
    }
}

impl Worker for Slot {
    fn is_reference_only(&self) -> bool {
        self.is_reference_only
    }
}

impl ID for Slot {
    fn id(&self) -> &str {
        &self.id
    }
}
