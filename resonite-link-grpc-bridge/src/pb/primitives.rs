use super::convert_utils::*;
use super::*;
use ::resonite_link_client::data_model;

impl_bi_into!(data_model::Float3, Float3, [x, y, z]);
impl_bi_into!(data_model::FloatQ, FloatQ, [x, y, z, w]);
impl_bi_into!(data_model::Color, Color, [r, g, b, a]);
impl_bi_into!(data_model::ColorX, ColorX, [r, g, b, a, profile]);
// TODO: Requires downscaling from 32 to 8 bits.
// impl_bi_into!(data_model::Color32, Color32, [r, g, b, a]);
