use core::fmt::{self, LowerExp, UpperExp};

use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue};

use super::base64;
use super::Num;

/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scalar {
    /// DynamoDB [string](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-S)
    /// value
    String(String),
    /// DynamoDB [numeric](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-N)
    /// value
    Num(Num),
    /// DynamoDB [boolean](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-BOOL)
    /// value
    Bool(bool),
    /// DynamoDB [binary](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-B)
    /// value
    Binary(Vec<u8>),
    /// DynamoDB [null](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-NULL)
    /// value
    Null,
}

impl Scalar {
    /// Use when you need a [string][1] value for DynamoDB.
    ///
    /// See also: [`Value::new_string`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-S
    /// [`Value::new_string`]: crate::value::Value::new_string
    pub fn new_string<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self::String(value.into())
    }

    /// Use when you need a [numeric][1] value for DynamoDB.
    ///
    /// See also: [`Scalar::new_num_lower_exp`], [`Scalar::new_num_upper_exp`],
    /// [`Value::new_num`], [`Num`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::Scalar;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let value = Scalar::new_num(2600);
    /// assert_eq!("2600", value.to_string());
    ///
    /// let value = Scalar::new_num(2600.0);
    /// assert_eq!("2600", value.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-N
    /// [`Value::new_num`]: crate::value::Value::new_num
    pub fn new_num<N>(value: N) -> Self
    where
        N: ToString + num::Num,
    {
        Self::Num(Num::new(value))
    }

    /// Use when you need a [numeric][1] value for DynamoDB in exponent form
    /// (with a lowercase `e`).
    ///
    /// See also: [`Scalar::new_num`], [`Scalar::new_num_upper_exp`],
    /// [`Value::new_num_lower_exp`], [`Num`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::Scalar;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let value = Scalar::new_num_lower_exp(2600);
    /// assert_eq!("2.6e3", value.to_string());
    ///
    /// let value = Scalar::new_num_lower_exp(2600.0);
    /// assert_eq!("2.6e3", value.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-N
    /// [`Value::new_num_lower_exp`]: crate::value::Value::new_num_lower_exp
    pub fn new_num_lower_exp<N>(value: N) -> Self
    where
        N: LowerExp + num::Num,
    {
        Self::Num(Num::new_lower_exp(value))
    }

    /// Use when you need a [numeric][1] value for DynamoDB in exponent form
    /// (with an uppercase `e`).
    ///
    /// See also: [`Scalar::new_num`], [`Scalar::new_num_lower_exp`],
    /// [`Value::new_num_upper_exp`], [`Num`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::Scalar;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let value = Scalar::new_num_upper_exp(2600);
    /// assert_eq!("2.6E3", value.to_string());
    ///
    /// let value = Scalar::new_num_upper_exp(2600.0);
    /// assert_eq!("2.6E3", value.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-N
    /// [`Value::new_num_upper_exp`]: crate::value::Value::new_num_upper_exp
    pub fn new_num_upper_exp<N>(value: N) -> Self
    where
        N: UpperExp + num::Num,
    {
        Self::Num(Num::new_upper_exp(value))
    }

    /// Use when you need a [boolean][1] value for DynamoDB.
    ///
    /// See also: [`Value::new_bool`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-BOOL
    /// [`Value::new_bool`]: crate::value::Value::new_bool
    pub fn new_bool(b: bool) -> Self {
        Self::Bool(b)
    }

    /// Use when you need a [binary][1] value for DynamoDB.
    ///
    /// See also: [`Value::new_binary`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-B
    /// [`Value::new_binary`]: crate::value::Value::new_binary
    pub fn new_binary<B>(binary: B) -> Self
    where
        B: Into<Vec<u8>>,
    {
        Self::Binary(binary.into())
    }

    /// Use when you need a [null][1] value for DynamoDB.
    ///
    /// See also: [`Value::new_null`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-NULL
    /// [`Value::new_null`]: crate::value::Value::new_null
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
            Self::String(s) => serde_json::to_string(s).unwrap().fmt(f),
            Self::Num(n) => n.fmt(f),
            Self::Bool(b) => serde_json::Value::Bool(*b).to_string().fmt(f),
            Self::Binary(b) => serde_json::Value::String(base64(b)).to_string().fmt(f),

            // TODO: I'm pretty sure this isn't right.
            // https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-NULL
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
        Self::String(value.to_owned())
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

#[cfg(test)]
mod test {
    use pretty_assertions::assert_str_eq;

    use super::Scalar;

    #[test]
    fn string() {
        let actual = Scalar::new_string("fish");
        assert_str_eq!("\"fish\"", actual.to_string());
    }

    #[test]
    fn numeric() {
        let actual = Scalar::new_num(42);
        assert_str_eq!("42", actual.to_string());
    }

    #[test]
    fn boolean() {
        assert_str_eq!("true", Scalar::new_bool(true).to_string());
        assert_str_eq!("false", Scalar::new_bool(false).to_string());
    }

    #[test]
    fn binary_vec() {
        let bytes: Vec<u8> = b"fish".to_vec();
        let actual = Scalar::new_binary(bytes);
        assert_str_eq!(r#""ZmlzaA==""#, actual.to_string());
    }

    #[test]
    fn binary_array() {
        let bytes: [u8; 4] = [b'f', b'i', b's', b'h'];
        let actual = Scalar::new_binary(bytes);
        assert_str_eq!(r#""ZmlzaA==""#, actual.to_string());
    }

    #[test]
    fn binary_slice() {
        let bytes: &[u8] = &b"fish"[..];
        let actual = Scalar::new_binary(bytes);
        assert_str_eq!(r#""ZmlzaA==""#, actual.to_string());
    }

    #[test]
    fn null() {
        let actual = Scalar::new_null();
        assert_str_eq!("NULL", actual.to_string());
    }
}
