use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct FloatQ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ColorX {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub profile: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Color32 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
