use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Int2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Int3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Int4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Float2 {
    #[serde(with = "super::floats::Ser")]
    pub x: f32,
    #[serde(with = "super::floats::Ser")]
    pub y: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Float3 {
    #[serde(with = "super::floats::Ser")]
    pub x: f32,
    #[serde(with = "super::floats::Ser")]
    pub y: f32,
    #[serde(with = "super::floats::Ser")]
    pub z: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Float4 {
    #[serde(with = "super::floats::Ser")]
    pub x: f32,
    #[serde(with = "super::floats::Ser")]
    pub y: f32,
    #[serde(with = "super::floats::Ser")]
    pub z: f32,
    #[serde(with = "super::floats::Ser")]
    pub w: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct FloatQ {
    #[serde(with = "super::floats::Ser")]
    pub x: f32,
    #[serde(with = "super::floats::Ser")]
    pub y: f32,
    #[serde(with = "super::floats::Ser")]
    pub z: f32,
    #[serde(with = "super::floats::Ser")]
    pub w: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Color {
    #[serde(with = "super::floats::Ser")]
    pub r: f32,
    #[serde(with = "super::floats::Ser")]
    pub g: f32,
    #[serde(with = "super::floats::Ser")]
    pub b: f32,
    #[serde(with = "super::floats::Ser")]
    pub a: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ColorX {
    #[serde(with = "super::floats::Ser")]
    pub r: f32,
    #[serde(with = "super::floats::Ser")]
    pub g: f32,
    #[serde(with = "super::floats::Ser")]
    pub b: f32,
    #[serde(with = "super::floats::Ser")]
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
