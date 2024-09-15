use core::fmt::{self, LowerExp, UpperExp};

use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue};

use super::{base64, Num};

/// Represents a DynamoDB [scalar value][1].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.Scalar
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scalar {
    /// DynamoDB [string value](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-S)
    String(String),

    /// DynamoDB [numeric value](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-N)
    Num(Num),

    /// DynamoDB [boolean value](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-BOOL)
    Bool(bool),

    /// DynamoDB [binary value](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-B)
    Binary(Vec<u8>),

    /// DynamoDB [null value](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-NULL)
    Null,
}

impl Scalar {
    /// Use when you need a [string value][1] for DynamoDB.
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

    /// Use when you need a [numeric value][1] for DynamoDB.
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

    /// Use when you need a [numeric value][1] for DynamoDB in exponent form
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

    /// Use when you need a [numeric value][1] for DynamoDB in exponent form
    /// (with an uppercase `E`).
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

    /// Use when you need a [boolean value][1] for DynamoDB.
    ///
    /// See also: [`Value::new_bool`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-BOOL
    /// [`Value::new_bool`]: crate::value::Value::new_bool
    pub fn new_bool(b: bool) -> Self {
        Self::Bool(b)
    }

    /// Use when you need a [binary value][1] for DynamoDB.
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

    /// Use when you need a [null value][1] for DynamoDB.
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
            Self::String(s) => serde_json::to_string(s).map_err(|_err| fmt::Error)?.fmt(f),
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

impl<const N: usize> From<&[u8; N]> for Scalar {
    fn from(value: &[u8; N]) -> Self {
        Self::Binary(value.into())
    }
}

impl From<&[u8]> for Scalar {
    fn from(value: &[u8]) -> Self {
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
    use pretty_assertions::assert_eq;

    use crate::Num;

    use super::Scalar;

    #[test]
    fn string() {
        let fish: &str = "fish";

        let actual = Scalar::new_string(fish);
        assert_eq!("\"fish\"", actual.to_string());

        // &str
        let actual = Scalar::from(fish);
        assert_eq!("\"fish\"", actual.to_string());

        // &&str
        let phish: &&str = &fish;
        let actual = Scalar::from(phish);
        assert_eq!("\"fish\"", actual.to_string());

        // &String
        let phish: String = fish.into();
        let actual = Scalar::from(&phish);
        assert_eq!("\"fish\"", actual.to_string());

        // String
        let actual = Scalar::from(phish);
        assert_eq!("\"fish\"", actual.to_string());
    }

    #[test]
    fn numeric() {
        let actual = Scalar::new_num(42);
        assert_eq!("42", actual.to_string());

        let actual = Scalar::from(Num::new(42));
        assert_eq!("42", actual.to_string());
    }

    #[test]
    fn boolean() {
        assert_eq!("true", Scalar::new_bool(true).to_string());
        assert_eq!("false", Scalar::new_bool(false).to_string());

        assert_eq!("true", Scalar::from(true).to_string());
        assert_eq!("false", Scalar::from(false).to_string());
    }

    #[test]
    fn binary_vec() {
        let bytes: Vec<u8> = b"fish".into();

        let actual = Scalar::new_binary(bytes.clone());
        assert_eq!(r#""ZmlzaA==""#, actual.to_string());

        let actual = Scalar::from(bytes);
        assert_eq!(r#""ZmlzaA==""#, actual.to_string());
    }

    #[test]
    fn binary_array() {
        let bytes: [u8; 4] = b"fish".to_owned();

        let actual = Scalar::new_binary(bytes);
        assert_eq!(r#""ZmlzaA==""#, actual.to_string());

        let actual = Scalar::from(bytes);
        assert_eq!(r#""ZmlzaA==""#, actual.to_string());
    }

    #[test]
    fn binary_array_ref() {
        let bytes: &[u8; 4] = b"fish";

        let actual = Scalar::new_binary(bytes);
        assert_eq!(r#""ZmlzaA==""#, actual.to_string());

        let actual = Scalar::from(bytes);
        assert_eq!(r#""ZmlzaA==""#, actual.to_string());
    }

    #[test]
    fn binary_slice() {
        let bytes: &[u8] = &b"fish"[..];

        let actual = Scalar::new_binary(bytes);
        assert_eq!(r#""ZmlzaA==""#, actual.to_string());

        let actual = Scalar::from(bytes);
        assert_eq!(r#""ZmlzaA==""#, actual.to_string());
    }

    #[test]
    fn null() {
        assert_eq!("NULL", Scalar::Null.to_string());
        assert_eq!("NULL", Scalar::new_null().to_string());
        assert_eq!("NULL", Scalar::from(()).to_string());
    }
}
