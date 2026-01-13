use serde::de::{Error, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[derive(PartialEq, Debug, Default, Copy, Clone)]
pub struct Float<T: FloatPrim>(T);

pub type F32 = Float<f32>;
pub type F64 = Float<f64>;

impl<T: FloatPrim> Float<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T: FloatPrim> Deref for Float<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: FloatPrim> DerefMut for Float<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: FloatPrim> From<T> for Float<T> {
    fn from(val: T) -> Self {
        Float(val)
    }
}

// These impls can't be generalized due to some constraint I don't understand.
impl From<Float<f32>> for f32 {
    fn from(val: Float<f32>) -> Self {
        val.0
    }
}

impl From<Float<f64>> for f64 {
    fn from(val: Float<f64>) -> Self {
        val.0
    }
}

pub trait FloatPrim: Clone {
    fn from_f64(val: f64) -> Self;
    fn as_f64(&self) -> f64;
    fn inf() -> Self;
    fn neg_inf() -> Self;
    fn nan() -> Self;
    fn is_inf(&self) -> bool;
    fn is_positive(&self) -> bool;
    fn is_val_nan(&self) -> bool;
}

impl FloatPrim for f32 {
    fn from_f64(val: f64) -> Self {
        val as f32
    }
    fn as_f64(&self) -> f64 {
        *self as f64
    }
    fn inf() -> Self {
        f32::INFINITY
    }
    fn neg_inf() -> Self {
        f32::NEG_INFINITY
    }
    fn nan() -> Self {
        f32::NAN
    }
    fn is_inf(&self) -> bool {
        self.is_infinite()
    }
    fn is_positive(&self) -> bool {
        self.is_sign_positive()
    }
    fn is_val_nan(&self) -> bool {
        self.is_nan()
    }
}

impl FloatPrim for f64 {
    fn from_f64(val: f64) -> Self {
        val
    }
    fn as_f64(&self) -> f64 {
        *self
    }
    fn inf() -> Self {
        f64::INFINITY
    }
    fn neg_inf() -> Self {
        f64::NEG_INFINITY
    }
    fn nan() -> Self {
        f64::NAN
    }
    fn is_inf(&self) -> bool {
        self.is_infinite()
    }
    fn is_positive(&self) -> bool {
        self.is_sign_positive()
    }
    fn is_val_nan(&self) -> bool {
        self.is_nan()
    }
}

pub struct Ser<T>(PhantomData<T>);

impl<T: FloatPrim> Ser<T> {
    pub fn serialize<S>(v: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if v.is_val_nan() {
            serializer.serialize_str("NaN")
        } else if v.is_inf() {
            if v.is_positive() {
                serializer.serialize_str("Infinity")
            } else {
                serializer.serialize_str("-Infinity")
            }
        } else {
            serializer.serialize_f64(v.as_f64())
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FloatSerdeVisitor<T: FloatPrim>(PhantomData<T>);
        impl<'de, T: FloatPrim> Visitor<'de> for FloatSerdeVisitor<T> {
            type Value = T;
            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a float, including Infinity, -Infinity, or NaN")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(T::from_f64(v as f64))
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                Ok(T::from_f64(v as f64))
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
                Ok(T::from_f64(v))
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match v {
                    "Infinity" => Ok(T::inf()),
                    "-Infinity" => Ok(T::neg_inf()),
                    "NaN" => Ok(T::nan()),
                    _ => Err(E::invalid_type(Unexpected::Str(v), &self)),
                }
            }
        }

        deserializer.deserialize_any(FloatSerdeVisitor(PhantomData::<T>::default()))
    }
}

impl<T: FloatPrim> Serialize for Float<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ser::serialize(&self.0, serializer)
    }
}

impl<'de, T: FloatPrim + Deserialize<'de>> Deserialize<'de> for Float<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ser::deserialize(deserializer).map(Float)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_bi_eq_json;
    use serde_json::{from_value, json, to_value};

    #[test]
    fn float_num() {
        assert_bi_eq_json(Float(1f32), json!(1.0));
    }

    #[test]
    fn nan() {
        assert_eq!(to_value(Float(f32::NAN)).unwrap(), json!("NaN"));
        assert!(from_value::<F32>(json!("NaN")).unwrap().is_nan());
    }

    #[test]
    fn inf() {
        assert_bi_eq_json(Float(f32::INFINITY), json!("Infinity"));
    }

    #[test]
    fn neg_inf() {
        assert_bi_eq_json(Float(f32::NEG_INFINITY), json!("-Infinity"));
    }
}
