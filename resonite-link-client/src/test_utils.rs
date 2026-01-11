use std::fmt::Debug;
use serde::de::DeserializeOwned;
use serde_json::{from_value, to_value};

pub fn assert_bi_eq_json<'de, T>(value: T, expected: serde_json::Value)
where T : serde::Serialize + DeserializeOwned + PartialEq + Debug {
    assert_eq!(to_value(&value).unwrap(), expected);
    assert_eq!(from_value::<T>(expected).unwrap(), value);
}