use alloc::borrow::Cow;
use core::fmt;

use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue};
use base64::{engine::general_purpose, Engine as _};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScalarValue {
    pub(crate) value: ScalarType,
}

impl ScalarValue {
    /// Use when you need a string value for DynamoDB.
    pub fn new_string<T>(value: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self {
            value: ScalarType::String(value.into()),
        }
    }

    /// Use when you need a numeric value for DynamoDB.
    pub fn new_num<N>(value: N) -> Self
    where
        N: ToString + num::Num,
    {
        Self {
            value: ScalarType::Num(value.to_string().into()),
        }
    }

    /// Use when you need a bool value for DynamoDB.
    pub fn new_bool(b: bool) -> Self {
        Self {
            value: ScalarType::Bool(b),
        }
    }

    /// Use when you need a binary value for DynamoDB.
    pub fn new_binary<B>(binary: B) -> ScalarValue
    where
        B: Into<Cow<'static, [u8]>>,
    {
        Self {
            value: ScalarType::Binary(binary.into()),
        }
    }

    /// Use when you need a NULL value for DynamoDB.
    pub fn new_null() -> Self {
        Self {
            value: ScalarType::Null,
        }
    }

    // Intentionally not using `impl From<ScalarValue> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    pub(crate) fn into_attribute_value(self) -> AttributeValue {
        self.value.into()
    }
}

impl fmt::Display for ScalarValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ScalarType::String(s) => s.fmt(f),
            ScalarType::Num(n) => n.fmt(f),
            ScalarType::Bool(b) => b.fmt(f),
            ScalarType::Binary(b) => general_purpose::STANDARD.encode(b).fmt(f),
            ScalarType::Null => f.write_str("NULL"),
        }
    }
}

/// https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum ScalarType {
    String(Cow<'static, str>),
    Num(Cow<'static, str>),
    Bool(bool),
    Binary(Cow<'static, [u8]>),
    Null,
}

impl From<ScalarType> for AttributeValue {
    fn from(value: ScalarType) -> Self {
        match value {
            ScalarType::String(s) => Self::S(s.into()),
            ScalarType::Num(n) => Self::N(n.into()),
            ScalarType::Bool(b) => Self::Bool(b),
            ScalarType::Binary(b) => Self::B(Blob::new(b)),
            ScalarType::Null => Self::Null(true),
        }
    }
}

/// Use when you need a string value for DynamoDB.
pub fn string_value<T>(value: T) -> ScalarValue
where
    T: Into<Cow<'static, str>>,
{
    ScalarValue::new_string(value)
}

/// Use when you need a numeric value for DynamoDB.
pub fn num_value<N>(value: N) -> ScalarValue
where
    N: ToString + num::Num,
{
    ScalarValue::new_num(value)
}

/// Use when you need a bool value for DynamoDB.
pub fn bool_value(b: bool) -> ScalarValue {
    ScalarValue::new_bool(b)
}

/// Use when you need a binary value for DynamoDB.
pub fn binary_value<B>(b: B) -> ScalarValue
where
    B: Into<Cow<'static, [u8]>>,
{
    ScalarValue::new_binary(b)
}

/// Use when you need a NULL value for DynamoDB.
pub fn null_value() -> ScalarValue {
    ScalarValue::new_null()
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_str_eq;

    use super::{binary_value, bool_value, null_value, num_value, string_value};

    #[test]
    fn string() {
        let actual = string_value("fish");
        assert_str_eq!("fish", actual.to_string());
    }

    #[test]
    fn numeral() {
        let actual = num_value(42);
        assert_str_eq!("42", actual.to_string());
    }

    #[test]
    fn boolean() {
        let actual = bool_value(true);
        assert_str_eq!("true", actual.to_string());

        let actual = bool_value(false);
        assert_str_eq!("false", actual.to_string());
    }

    #[test]
    fn binary_vec() {
        let actual = binary_value(b"fish".to_vec());
        assert_str_eq!("ZmlzaA==", actual.to_string());
    }

    // #[test]
    // fn binary_array() {
    //     let actual = binary_value(b"fish");
    //     assert_str_eq!("ZmlzaA==", actual.to_string());
    // }

    // #[test]
    // fn binary_slice() {
    //     let actual = binary_value(&b"fish");
    //     assert_str_eq!("ZmlzaA==", actual.to_string());
    // }

    #[test]
    fn null() {
        let actual = null_value();
        assert_str_eq!("NULL", actual.to_string());
    }
}
