pub mod list;
pub mod map;
pub mod scalar;
pub mod set;
mod value_or_ref;

pub use list::List;
pub use map::Map;
pub use scalar::{binary_value, bool_value, null_value, num_value, string_value, Num, Scalar};
pub use set::{binary_set, num_set, string_set, BinarySet, NumSet, Set, StringSet};
pub use value_or_ref::{ref_value, Ref};

pub(crate) use value_or_ref::{StringOrRef, ValueOrRef};

use core::fmt;

use aws_sdk_dynamodb::types::AttributeValue;
use base64::{engine::general_purpose, Engine as _};

/// A DynamoDB value
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Scalar(Scalar),
    Set(Set),
    Map(Map),
    List(List),
}

impl Value {
    // Intentionally not using `impl From<ScalarValue> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    pub(crate) fn into_attribute_value(self) -> AttributeValue {
        match self {
            Self::Scalar(value) => value.into_attribute_value(),
            Self::Set(value) => value.into_attribute_value(),
            Self::Map(value) => value.into_attribute_value(),
            Self::List(value) => value.into_attribute_value(),
        }
    }
}

impl<T> From<T> for Value
where
    T: Into<Scalar>,
{
    fn from(scalar: T) -> Self {
        Self::Scalar(scalar.into())
    }
}

impl From<Set> for Value {
    fn from(set: Set) -> Self {
        Self::Set(set)
    }
}

impl From<Map> for Value {
    fn from(map: Map) -> Self {
        Self::Map(map)
    }
}

impl From<List> for Value {
    fn from(list: List) -> Self {
        Self::List(list)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scalar(value) => value.fmt(f),
            Self::Set(value) => value.fmt(f),
            Self::Map(value) => value.fmt(f),
            Self::List(value) => value.fmt(f),
        }
    }
}

/// Produces base64 the way DynamoDB wants it.
pub(crate) fn base64<T>(b: T) -> String
where
    T: AsRef<[u8]>,
{
    general_purpose::STANDARD.encode(b)
}
