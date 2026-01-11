use crate::data_model::{Component, Slot};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(flatten, default = "ResponseKind::Response")]
    pub kind: ResponseKind,

    pub source_message_id: Option<String>,
    pub success: bool,
    pub error_info: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase", tag = "$type")]
pub enum ResponseKind {
    Response,
    #[serde(rename_all = "camelCase")]
    SlotData {
        depth: i32,
        data: Option<Slot>,
    },
    #[serde(rename_all = "camelCase")]
    ComponentData {
        data: Option<Component>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn test_response() {
        assert!(
            // This is a GetData request from a new gridspace world.
            serde_json::from_value::<Response>(json!({
              "$type": "slotData",
              "depth": 0,
              "data": {
                "parent": {
                  "targetId": null,
                  "targetType": "FrooxEngine.Slot",
                  "id": "Reso_0"
                },
                "position": { "value": { "x": 0, "y": 0, "z": 0 }, "id": "Reso_1" },
                "rotation": { "value": { "x": 0, "y": 0, "z": 0, "w": 1 }, "id": "Reso_2" },
                "scale": { "value": { "x": 1, "y": 1, "z": 1 }, "id": "Reso_3" },
                "isActive": { "value": true, "id": "Reso_4" },
                "isPersistent": { "value": true, "id": "Reso_5" },
                "name": { "value": "Root", "id": "Reso_6" },
                "tag": { "value": null, "id": "Reso_7" },
                "orderOffset": { "value": 0, "id": "Reso_8" },
                "components": [
                  {
                    "componentType": "FrooxEngine.StaticLocaleProvider",
                    "members": null,
                    "id": "Reso_9",
                    "isReferenceOnly": true
                  },
                  {
                    "componentType": "FrooxEngine.AssetLoader<FrooxEngine.LocaleResource>",
                    "members": null,
                    "id": "Reso_A",
                    "isReferenceOnly": true
                  },
                  {
                    "componentType": "FrooxEngine.GradientStripTexture",
                    "members": null,
                    "id": "Reso_B",
                    "isReferenceOnly": true
                  },
                  {
                    "componentType": "FrooxEngine.DynamicVariableSpace",
                    "members": null,
                    "id": "Reso_C",
                    "isReferenceOnly": true
                  },
                  {
                    "componentType": "FrooxEngine.LocomotionAnimationConfiguration",
                    "members": null,
                    "id": "Reso_D",
                    "isReferenceOnly": true
                  },
                  {
                    "componentType": "FrooxEngine.FingerPosePreset",
                    "members": null,
                    "id": "Reso_E",
                    "isReferenceOnly": true
                  },
                  {
                    "componentType": "FrooxEngine.FingerPosePreset",
                    "members": null,
                    "id": "Reso_F",
                    "isReferenceOnly": true
                  },
                  {
                    "componentType": "FrooxEngine.FontChain",
                    "members": null,
                    "id": "Reso_10",
                    "isReferenceOnly": true
                  },
                  {
                    "componentType": "FrooxEngine.StaticFont",
                    "members": null,
                    "id": "Reso_11",
                    "isReferenceOnly": true
                  },
                  {
                    "componentType": "FrooxEngine.StaticFont",
                    "members": null,
                    "id": "Reso_12",
                    "isReferenceOnly": true
                  },
                  {
                    "componentType": "FrooxEngine.StaticFont",
                    "members": null,
                    "id": "Reso_13",
                    "isReferenceOnly": true
                  },
                  {
                    "componentType": "FrooxEngine.StaticFont",
                    "members": null,
                    "id": "Reso_14",
                    "isReferenceOnly": true
                  },
                  {
                    "componentType": "FrooxEngine.StaticFont",
                    "members": null,
                    "id": "Reso_15",
                    "isReferenceOnly": true
                  },
                  {
                    "componentType": "FrooxEngine.StaticFont",
                    "members": null,
                    "id": "Reso_16",
                    "isReferenceOnly": true
                  },
                  {
                    "componentType": "FrooxEngine.StaticFont",
                    "members": null,
                    "id": "Reso_17",
                    "isReferenceOnly": true
                  },
                  {
                    "componentType": "FrooxEngine.TextUnlitMaterial",
                    "members": null,
                    "id": "Reso_18",
                    "isReferenceOnly": true
                  },
                  {
                    "componentType": "FrooxEngine.DynamicValueVariable<string>",
                    "members": null,
                    "id": "Reso_19",
                    "isReferenceOnly": true
                  }
                ],
                "children": [
                  {
                    "parent": {
                      "targetId": "Root",
                      "targetType": "FrooxEngine.Slot",
                      "id": "Reso_1B"
                    },
                    "position": { "value": { "x": 0, "y": 0, "z": 0 }, "id": "Reso_1C" },
                    "rotation": {
                      "value": { "x": 0, "y": 0, "z": 0, "w": 1 },
                      "id": "Reso_1D"
                    },
                    "scale": { "value": { "x": 1, "y": 1, "z": 1 }, "id": "Reso_1E" },
                    "isActive": { "value": true, "id": "Reso_1F" },
                    "isPersistent": { "value": true, "id": "Reso_20" },
                    "name": { "value": "Controllers", "id": "Reso_21" },
                    "tag": { "value": null, "id": "Reso_22" },
                    "orderOffset": { "value": 0, "id": "Reso_23" },
                    "components": null,
                    "children": null,
                    "id": "Reso_1A",
                    "isReferenceOnly": true
                  },
                  {
                    "parent": {
                      "targetId": "Root",
                      "targetType": "FrooxEngine.Slot",
                      "id": "Reso_25"
                    },
                    "position": { "value": { "x": 0, "y": 0, "z": 0 }, "id": "Reso_26" },
                    "rotation": {
                      "value": { "x": 0, "y": 0, "z": 0, "w": 1 },
                      "id": "Reso_27"
                    },
                    "scale": { "value": { "x": 1, "y": 1, "z": 1 }, "id": "Reso_28" },
                    "isActive": { "value": true, "id": "Reso_29" },
                    "isPersistent": { "value": true, "id": "Reso_2A" },
                    "name": { "value": "Roles", "id": "Reso_2B" },
                    "tag": { "value": null, "id": "Reso_2C" },
                    "orderOffset": { "value": 0, "id": "Reso_2D" },
                    "components": null,
                    "children": null,
                    "id": "Reso_24",
                    "isReferenceOnly": true
                  },
                  {
                    "parent": {
                      "targetId": "Root",
                      "targetType": "FrooxEngine.Slot",
                      "id": "Reso_2F"
                    },
                    "position": { "value": { "x": 0, "y": 0, "z": 0 }, "id": "Reso_30" },
                    "rotation": {
                      "value": { "x": 0, "y": 0, "z": 0, "w": 1 },
                      "id": "Reso_31"
                    },
                    "scale": { "value": { "x": 1, "y": 1, "z": 1 }, "id": "Reso_32" },
                    "isActive": { "value": true, "id": "Reso_33" },
                    "isPersistent": { "value": true, "id": "Reso_34" },
                    "name": { "value": "Clipboard Importer", "id": "Reso_35" },
                    "tag": { "value": null, "id": "Reso_36" },
                    "orderOffset": { "value": 0, "id": "Reso_37" },
                    "components": null,
                    "children": null,
                    "id": "Reso_2E",
                    "isReferenceOnly": true
                  },
                  {
                    "parent": {
                      "targetId": "Root",
                      "targetType": "FrooxEngine.Slot",
                      "id": "Reso_39"
                    },
                    "position": { "value": { "x": 0, "y": 0.01, "z": 0 }, "id": "Reso_3A" },
                    "rotation": {
                      "value": { "x": 0, "y": 0, "z": 0, "w": 1 },
                      "id": "Reso_3B"
                    },
                    "scale": { "value": { "x": 1, "y": 1, "z": 1 }, "id": "Reso_3C" },
                    "isActive": { "value": true, "id": "Reso_3D" },
                    "isPersistent": { "value": true, "id": "Reso_3E" },
                    "name": { "value": "SpawnArea", "id": "Reso_3F" },
                    "tag": { "value": null, "id": "Reso_40" },
                    "orderOffset": { "value": 0, "id": "Reso_41" },
                    "components": null,
                    "children": null,
                    "id": "Reso_38",
                    "isReferenceOnly": true
                  },
                  {
                    "parent": {
                      "targetId": "Root",
                      "targetType": "FrooxEngine.Slot",
                      "id": "Reso_43"
                    },
                    "position": { "value": { "x": 0, "y": 2, "z": 0 }, "id": "Reso_44" },
                    "rotation": {
                      "value": {
                        "x": 0.9914449,
                        "y": -3.7631953e-8,
                        "z": -3.7631953e-8,
                        "w": 0.1305262
                      },
                      "id": "Reso_45"
                    },
                    "scale": { "value": { "x": 1, "y": 1, "z": 1 }, "id": "Reso_46" },
                    "isActive": { "value": true, "id": "Reso_47" },
                    "isPersistent": { "value": true, "id": "Reso_48" },
                    "name": { "value": "Light", "id": "Reso_49" },
                    "tag": { "value": null, "id": "Reso_4A" },
                    "orderOffset": { "value": 0, "id": "Reso_4B" },
                    "components": null,
                    "children": null,
                    "id": "Reso_42",
                    "isReferenceOnly": true
                  },
                  {
                    "parent": {
                      "targetId": "Root",
                      "targetType": "FrooxEngine.Slot",
                      "id": "Reso_4D"
                    },
                    "position": { "value": { "x": 0, "y": 0, "z": 0 }, "id": "Reso_4E" },
                    "rotation": {
                      "value": { "x": 0, "y": 0, "z": 0, "w": 1 },
                      "id": "Reso_4F"
                    },
                    "scale": { "value": { "x": 1, "y": 1, "z": 1 }, "id": "Reso_50" },
                    "isActive": { "value": true, "id": "Reso_51" },
                    "isPersistent": { "value": true, "id": "Reso_52" },
                    "name": { "value": "Skybox", "id": "Reso_53" },
                    "tag": { "value": null, "id": "Reso_54" },
                    "orderOffset": { "value": 0, "id": "Reso_55" },
                    "components": null,
                    "children": null,
                    "id": "Reso_4C",
                    "isReferenceOnly": true
                  },
                  {
                    "parent": {
                      "targetId": "Root",
                      "targetType": "FrooxEngine.Slot",
                      "id": "Reso_57"
                    },
                    "position": { "value": { "x": 0, "y": 0, "z": 0 }, "id": "Reso_58" },
                    "rotation": {
                      "value": { "x": 0, "y": 0, "z": 0, "w": 1 },
                      "id": "Reso_59"
                    },
                    "scale": { "value": { "x": 1, "y": 1, "z": 1 }, "id": "Reso_5A" },
                    "isActive": { "value": true, "id": "Reso_5B" },
                    "isPersistent": { "value": true, "id": "Reso_5C" },
                    "name": { "value": "Ground", "id": "Reso_5D" },
                    "tag": { "value": null, "id": "Reso_5E" },
                    "orderOffset": { "value": 0, "id": "Reso_5F" },
                    "components": null,
                    "children": null,
                    "id": "Reso_56",
                    "isReferenceOnly": true
                  },
                  {
                    "parent": {
                      "targetId": "Root",
                      "targetType": "FrooxEngine.Slot",
                      "id": "Reso_61"
                    },
                    "position": {
                      "value": { "x": -4.6393156, "y": -0.0002755938, "z": 2.7489536 },
                      "id": "Reso_62"
                    },
                    "rotation": {
                      "value": { "x": 0, "y": -0.00049950357, "z": 0, "w": 0.9999999 },
                      "id": "Reso_63"
                    },
                    "scale": { "value": { "x": 1, "y": 1, "z": 1 }, "id": "Reso_64" },
                    "isActive": { "value": true, "id": "Reso_65" },
                    "isPersistent": { "value": false, "id": "Reso_66" },
                    "name": {
                      "value": "User <noparse=9>Earthmark (ID2E00)",
                      "id": "Reso_67"
                    },
                    "tag": { "value": null, "id": "Reso_68" },
                    "orderOffset": { "value": 0, "id": "Reso_69" },
                    "components": null,
                    "children": null,
                    "id": "Reso_60",
                    "isReferenceOnly": true
                  },
                  {
                    "parent": {
                      "targetId": "Root",
                      "targetType": "FrooxEngine.Slot",
                      "id": "Reso_6B"
                    },
                    "position": { "value": { "x": 0, "y": 0, "z": 0 }, "id": "Reso_6C" },
                    "rotation": {
                      "value": { "x": 0, "y": 0, "z": 0, "w": 1 },
                      "id": "Reso_6D"
                    },
                    "scale": { "value": { "x": 1, "y": 1, "z": 1 }, "id": "Reso_6E" },
                    "isActive": { "value": true, "id": "Reso_6F" },
                    "isPersistent": { "value": false, "id": "Reso_70" },
                    "name": { "value": "__TEMP", "id": "Reso_71" },
                    "tag": { "value": null, "id": "Reso_72" },
                    "orderOffset": { "value": 0, "id": "Reso_73" },
                    "components": null,
                    "children": null,
                    "id": "Reso_6A",
                    "isReferenceOnly": true
                  },
                  {
                    "parent": {
                      "targetId": "Root",
                      "targetType": "FrooxEngine.Slot",
                      "id": "Reso_75"
                    },
                    "position": { "value": { "x": 0, "y": 0, "z": 0 }, "id": "Reso_76" },
                    "rotation": {
                      "value": { "x": 0, "y": 0, "z": 0, "w": 1 },
                      "id": "Reso_77"
                    },
                    "scale": { "value": { "x": 1, "y": 1, "z": 1 }, "id": "Reso_78" },
                    "isActive": { "value": true, "id": "Reso_79" },
                    "isPersistent": { "value": false, "id": "Reso_7A" },
                    "name": { "value": "Undo Manager", "id": "Reso_7B" },
                    "tag": { "value": null, "id": "Reso_7C" },
                    "orderOffset": { "value": 0, "id": "Reso_7D" },
                    "components": null,
                    "children": null,
                    "id": "Reso_74",
                    "isReferenceOnly": true
                  },
                  {
                    "parent": {
                      "targetId": "Root",
                      "targetType": "FrooxEngine.Slot",
                      "id": "Reso_7F"
                    },
                    "position": { "value": { "x": 0, "y": 0, "z": 0 }, "id": "Reso_80" },
                    "rotation": {
                      "value": { "x": 0, "y": 0, "z": 0, "w": 1 },
                      "id": "Reso_81"
                    },
                    "scale": { "value": { "x": 1, "y": 1, "z": 1 }, "id": "Reso_82" },
                    "isActive": { "value": true, "id": "Reso_83" },
                    "isPersistent": { "value": true, "id": "Reso_84" },
                    "name": { "value": "Assets", "id": "Reso_85" },
                    "tag": { "value": null, "id": "Reso_86" },
                    "orderOffset": { "value": 0, "id": "Reso_87" },
                    "components": null,
                    "children": null,
                    "id": "Reso_7E",
                    "isReferenceOnly": true
                  }
                ],
                "id": "Root",
                "isReferenceOnly": false
              },
              "sourceMessageId": "RS_REPL_BEF43B49_0",
              "success": true,
              "errorInfo": null
            }))
            .is_ok()
        );
    }
}
