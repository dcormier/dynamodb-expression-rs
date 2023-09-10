pub mod scalar;
pub mod set;

pub use scalar::{binary_value, bool_value, null_value, num_value, string_value, ScalarValue};
pub use set::{
    binary_set_value, num_set_value, string_set_value, BinarySet, NumSet, SetValue, StringSet,
};

use aws_sdk_dynamodb::types::AttributeValue;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Value {
    pub(crate) value: ValueType,
}

impl From<ValueType> for Value {
    fn from(value: ValueType) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ValueType {
    Scalar(ScalarValue),
    Set(SetValue),
}

impl From<ScalarValue> for ValueType {
    fn from(value: ScalarValue) -> Self {
        Self::Scalar(value)
    }
}

impl From<SetValue> for ValueType {
    fn from(value: SetValue) -> Self {
        Self::Set(value)
    }
}

// Using `From` only because `ValueType` is not public.
impl From<ValueType> for AttributeValue {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::Scalar(value) => value.into_attribute_value(),
            ValueType::Set(value) => value.into_attribute_value(),
        }
    }
}
