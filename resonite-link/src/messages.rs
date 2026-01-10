use serde::{Deserialize, Serialize};

use super::data_model::Component;
use super::data_model::Slot;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageWrapper {
    /// The kind of message to execute.
    #[serde(flatten)]
    pub inner: Message,

    /// Unique ID of this message. This can be used to match the response.
    pub message_id: String,
}

impl Message {
    pub fn new_get_slot(
        slot_id: impl Into<String>,
        depth: i32,
        include_component_data: bool,
    ) -> Self {
        Self::GetSlot {
            slot_id: slot_id.into(),
            depth,
            include_component_data,
        }
    }

    pub fn new_add_slot(slot: Slot) -> Self {
        Self::AddSlot { data: slot }
    }

    pub fn new_update_slot(slot: Slot) -> Self {
        Self::UpdateSlot { data: slot }
    }

    pub fn new_remove_slot(slot_id: impl Into<String>) -> Self {
        Self::RemoveSlot {
            slot_id: slot_id.into(),
        }
    }

    pub fn new_get_component(component_id: impl Into<String>) -> Self {
        Self::GetComponent {
            component_id: component_id.into(),
        }
    }

    pub fn new_add_component(component: Component, container_slot_id: impl Into<String>) -> Self {
        Self::AddComponent {
            data: component,
            container_slot_id: container_slot_id.into(),
        }
    }

    pub fn new_update_component(component: Component) -> Self {
        Self::UpdateComponent { data: component }
    }

    pub fn new_remove_component(component_id: impl Into<String>) -> Self {
        Self::RemoveComponent {
            component_id: component_id.into(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase", tag = "$type")]
pub enum Message {
    #[serde(rename_all = "camelCase")]
    GetSlot {
        /// Unique ID of the slot we're requesting data for.
        /// Special case: "Root" will fetch the root slot of the world.
        slot_id: String,
        /// How deep to fetch the hierarchy.
        /// Value of 0 will fetch only the requested slot fully.
        /// Value of 1 will fully fetch the immediate children.
        /// Value of -1 will fetch everything fully.
        /// Any immediate children of slots beyond this depth will be fetched as references only.
        depth: i32,
        /// Indicates if components should be fetched fully with all their data or only as references.
        /// Set to False if you plan on fetching the individual component data later.
        include_component_data: bool,
    },
    AddSlot {
        /// Data of the slot to set/update.
        /// When updating Slot, the ID must be specified.
        /// Any fields that are null will be left as is.
        data: Slot,
    },
    UpdateSlot {
        /// Data of the slot to set/update.
        /// When updating Slot, the ID must be specified.
        /// Any fields that are null will be left as is.
        data: Slot,
    },
    #[serde(rename_all = "camelCase")]
    RemoveSlot {
        /// Unique ID of the slot we're requesting data for.
        /// Special case: "Root" will fetch the root slot of the world.
        slot_id: String,
    },

    #[serde(rename_all = "camelCase")]
    GetComponent {
        /// The state of the component data. Any members that are not included will be left as is.
        /// When updating the component, the ID must be specified!
        component_id: String,
    },
    #[serde(rename_all = "camelCase")]
    AddComponent {
        /// The state of the component data. Any members that are not included will be left as is.
        /// When updating the component, the ID must be specified!
        data: Component,
        /// The ID of the Slot that this component should be added to.
        container_slot_id: String,
    },
    #[serde(rename_all = "camelCase")]
    UpdateComponent {
        /// The state of the component data. Any members that are not included will be left as is.
        /// When updating the component, the ID must be specified!
        data: Component,
    },
    #[serde(rename_all = "camelCase")]
    RemoveComponent {
        /// The ID of the component that's being removed
        component_id: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_bi_eq_json;
    use serde_json::json;

    #[test]
    fn serialize_get_slot() {
        assert_bi_eq_json(
            MessageWrapper {
                message_id: "Magic!".into(),
                inner: Message::new_get_slot("1", -1, false),
            },
            json!({
                "$type": "getSlot",
                "messageId": "Magic!",
                "slotId": "1",
                "depth": -1,
                "includeComponentData": false,
            }),
        );
    }
}
