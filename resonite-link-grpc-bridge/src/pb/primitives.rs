use super::convert_utils::*;
use super::*;
use ::resonite_link_client::data_model;

impl_bi_into!(data_model::Int2, Int2, [x, y]);
impl_bi_into!(data_model::Int3, Int3, [x, y, z]);
impl_bi_into!(data_model::Int4, Int4, [x, y, z, w]);
impl_bi_into!(data_model::Float2, Float2, [x, y]);
impl_bi_into!(data_model::Float3, Float3, [x, y, z]);
impl_bi_into!(data_model::Float4, Float4, [x, y, z, w]);
impl_bi_into!(data_model::FloatQ, FloatQ, [x, y, z, w]);
impl_bi_into!(data_model::Color, Color, [r, g, b, a]);
impl_bi_into!(data_model::ColorX, ColorX, [r, g, b, a, profile]);
impl_bi_into!(data_model::Color32, Color32, [r, g, b, a]);
