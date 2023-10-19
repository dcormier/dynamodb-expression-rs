use core::fmt;

use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue};

use super::base64;

/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scalar {
    String(String),
    Num(Num),
    Bool(bool),
    Binary(Vec<u8>),
    Null,
}

impl Scalar {
    /// Use when you need a string value for DynamoDB.
    pub fn new_string<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self::String(value.into())
    }

    /// Use when you need a numeric value for DynamoDB.
    pub fn new_num<N>(value: N) -> Self
    where
        N: ToString + num::Num,
    {
        Self::Num(value.into())
    }

    /// Use when you need a bool value for DynamoDB.
    pub fn new_bool(b: bool) -> Self {
        Self::Bool(b)
    }

    /// Use when you need a binary value for DynamoDB.
    pub fn new_binary<B>(binary: B) -> Self
    where
        B: Into<Vec<u8>>,
    {
        Self::Binary(binary.into())
    }

    /// Use when you need a NULL value for DynamoDB.
    pub fn new_null() -> Self {
        Self::Null
    }

    // Intentionally not using `impl From<Scalar> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    pub(super) fn into_attribute_value(self) -> AttributeValue {
        match self {
            Scalar::String(s) => AttributeValue::S(s),
            Scalar::Num(n) => n.into_attribute_value(),
            Scalar::Bool(b) => AttributeValue::Bool(b),
            Scalar::Binary(b) => AttributeValue::B(Blob::new(b)),
            Scalar::Null => AttributeValue::Null(true),
        }
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // TODO: Is JSON encoding this correct?
            Self::String(s) => serde_json::to_string(s).unwrap().fmt(f),
            Self::Num(n) => n.fmt(f),
            Self::Bool(b) => serde_json::Value::Bool(*b).to_string().fmt(f),
            Self::Binary(b) => serde_json::Value::String(base64(b)).to_string().fmt(f),
            Self::Null => f.write_str("NULL"),
        }
    }
}

impl From<String> for Scalar {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&String> for Scalar {
    fn from(value: &String) -> Self {
        Self::String(value.clone())
    }
}

impl From<&str> for Scalar {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<&&str> for Scalar {
    fn from(value: &&str) -> Self {
        Self::String((*value).to_owned())
    }
}

impl From<Num> for Scalar {
    fn from(value: Num) -> Self {
        Self::Num(value)
    }
}

impl From<bool> for Scalar {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<Vec<u8>> for Scalar {
    fn from(value: Vec<u8>) -> Self {
        Self::Binary(value)
    }
}

impl<const N: usize> From<[u8; N]> for Scalar {
    fn from(value: [u8; N]) -> Self {
        Self::Binary(value.into())
    }
}

impl From<()> for Scalar {
    fn from(_: ()) -> Self {
        Self::Null
    }
}

impl FromIterator<u8> for Scalar {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = u8>,
    {
        Self::Binary(iter.into_iter().collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Num {
    n: String,
}

impl Num {
    // Intentionally not using `impl From<Num> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    pub(super) fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::N(self.n)
    }
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.n.fmt(f)
    }
}

impl<N> From<N> for Num
where
    N: ToString + num::Num,
{
    fn from(value: N) -> Self {
        Self {
            n: value.to_string(),
        }
    }
}

/// Use when you need a string value for DynamoDB.
pub fn string_value<T>(value: T) -> Scalar
where
    T: Into<String>,
{
    Scalar::new_string(value)
}

/// Use when you need a numeric value for DynamoDB.
pub fn num_value<N>(value: N) -> Scalar
where
    N: ToString + num::Num,
{
    Scalar::new_num(value)
}

/// Use when you need a bool value for DynamoDB.
pub fn bool_value(b: bool) -> Scalar {
    Scalar::new_bool(b)
}

/// Use when you need a binary value for DynamoDB.
pub fn binary_value<B>(b: B) -> Scalar
where
    B: Into<Vec<u8>>,
{
    Scalar::new_binary(b)
}

/// Use when you need a NULL value for DynamoDB.
pub fn null_value() -> Scalar {
    Scalar::new_null()
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_str_eq;

    use super::{binary_value, bool_value, null_value, num_value, string_value};

    #[test]
    fn string() {
        let actual = string_value("fish");
        assert_str_eq!("\"fish\"", actual.to_string());
    }

    #[test]
    fn numeric() {
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
        let bytes: Vec<u8> = b"fish".to_vec();
        let actual = binary_value(bytes);
        assert_str_eq!(r#""ZmlzaA==""#, actual.to_string());
    }

    #[test]
    fn binary_array() {
        let bytes: [u8; 4] = [b'f', b'i', b's', b'h'];
        let actual = binary_value(bytes);
        assert_str_eq!(r#""ZmlzaA==""#, actual.to_string());
    }

    #[test]
    fn binary_slice() {
        let bytes: &[u8] = &b"fish"[..];
        let actual = binary_value(bytes);
        assert_str_eq!(r#""ZmlzaA==""#, actual.to_string());
    }

    #[test]
    fn null() {
        let actual = null_value();
        // TODO: Is this right?
        assert_str_eq!("NULL", actual.to_string());
    }
}
