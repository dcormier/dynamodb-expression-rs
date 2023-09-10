use core::fmt;

use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue};
use base64::{engine::general_purpose, Engine as _};

use super::{Value, ValueType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SetValue {
    StringSet(StringSet),
    NumSet(NumSet),
    BinarySet(BinarySet),
}

impl SetValue {
    pub(crate) fn into_value(self) -> Value {
        ValueType::from(self).into()
    }

    // Intentionally not using `impl From<SetValue> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    pub(crate) fn into_attribute_value(self) -> AttributeValue {
        match self {
            SetValue::StringSet(set) => set.into_attribute_value(),
            SetValue::NumSet(set) => set.into_attribute_value(),
            SetValue::BinarySet(set) => set.into_attribute_value(),
        }
    }
}

impl fmt::Display for SetValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetValue::StringSet(set) => set.fmt(f),
            SetValue::NumSet(set) => set.fmt(f),
            SetValue::BinarySet(set) => set.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringSet(Vec<String>);

impl StringSet {
    // Intentionally not using `impl From<StringSet> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::Ss(self.0)
    }
}

impl<I, T> From<I> for StringSet
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    fn from(values: I) -> Self {
        Self(values.into_iter().map(Into::into).collect())
    }
}

impl fmt::Display for StringSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

pub fn string_set_value<I, T>(set: I) -> SetValue
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    SetValue::StringSet(set.into())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumSet(Vec<String>);

impl NumSet {
    pub fn push<T>(&mut self, num: T)
    where
        T: ToString + num::Num,
    {
        self.0.push(Self::into_num(num));
    }

    /// Converts a numeric type into a DynamoDB numeric value
    fn into_num<T>(num: T) -> String
    where
        T: ToString + num::Num,
    {
        num.to_string()
    }

    // Intentionally not using `impl From<NumSet> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::Ns(self.0)
    }
}

impl<I, T> From<I> for NumSet
where
    I: IntoIterator<Item = T>,
    T: ToString + num::Num,
{
    fn from(values: I) -> Self {
        Self(values.into_iter().map(Self::into_num).collect())
    }
}

impl fmt::Display for NumSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

pub fn num_set_value<I, T>(set: I) -> SetValue
where
    I: IntoIterator<Item = T>,
    T: ToString + num::Num,
{
    SetValue::NumSet(set.into())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinarySet(Vec<Vec<u8>>);

impl BinarySet {
    // Intentionally not using `impl From<BinarySet> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::Bs(self.0.into_iter().map(Blob::new).collect())
    }
}

impl<I, T> From<I> for BinarySet
where
    I: IntoIterator<Item = T>,
    T: IntoIterator<Item = u8>,
{
    fn from(values: I) -> Self {
        Self(
            values
                .into_iter()
                .map(|value| value.into_iter().collect())
                .collect(),
        )
    }
}

impl fmt::Display for BinarySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.0.iter().map(|b| general_purpose::STANDARD.encode(b)))
            .finish()
    }
}

pub fn binary_set_value<I, T>(set: I) -> SetValue
where
    I: IntoIterator<Item = T>,
    T: IntoIterator<Item = u8>,
{
    SetValue::BinarySet(set.into())
}
