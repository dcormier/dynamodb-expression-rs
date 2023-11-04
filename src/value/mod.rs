pub mod list;
pub mod map;
mod num;
pub mod scalar;
pub mod set;
mod value_or_ref;

pub use list::List;
pub use map::Map;
pub use num::Num;
pub use scalar::Scalar;
pub use set::{BinarySet, NumSet, Set, StringSet};
pub use value_or_ref::Ref;

pub(crate) use value_or_ref::{StringOrRef, ValueOrRef};

use core::fmt::{self, LowerExp, UpperExp};

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

/// Scalar values
impl Value {
    /// Use when you need a [string][1] value for DynamoDB.
    ///
    /// See also: [`Scalar::new_string`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-S
    pub fn new_string<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self::Scalar(value.into().into())
    }

    /// Use when you need a [numeric][1] value for DynamoDB.
    ///
    /// See also:, [`Value::new_num_lower_exp`], [`Value::new_num_upper_exp`], [`Num`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::Value;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let value = Value::new_num(2600);
    /// assert_eq!("2600", value.to_string());
    ///
    /// let value = Value::new_num(2600.0);
    /// assert_eq!("2600", value.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-N
    pub fn new_num<N>(value: N) -> Self
    where
        N: ToString + ::num::Num,
    {
        Self::Scalar(Num::new(value).into())
    }

    /// Use when you need a [numeric][1] value for DynamoDB in exponent form
    /// (with a lowercase `e`).
    ///
    /// See also:, [`Value::new_num`], [`Value::new_num_upper_exp`], [`Num`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::Value;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let value = Value::new_num_lower_exp(2600);
    /// assert_eq!("2.6e3", value.to_string());
    ///
    /// let value = Value::new_num_lower_exp(2600.0);
    /// assert_eq!("2.6e3", value.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-N
    pub fn new_num_lower_exp<N>(value: N) -> Self
    where
        N: LowerExp + ::num::Num,
    {
        Self::Scalar(Num::new_lower_exp(value).into())
    }

    /// Use when you need a [numeric][1] value for DynamoDB in exponent form
    /// (with an uppercase `e`).
    ///
    /// See also:, [`Value::new_num`], [`Value::new_num_lower_exp`], [`Num`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::Value;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let value = Value::new_num_upper_exp(2600);
    /// assert_eq!("2.6E3", value.to_string());
    ///
    /// let value = Value::new_num_upper_exp(2600.0);
    /// assert_eq!("2.6E3", value.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-N
    pub fn new_num_upper_exp<N>(value: N) -> Self
    where
        N: UpperExp + ::num::Num,
    {
        Self::Scalar(Num::new_upper_exp(value).into())
    }

    /// Use when you need a [boolean][1] value for DynamoDB.
    ///
    /// See also: [`Scalar::new_bool`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-BOOL
    pub fn new_bool(b: bool) -> Self {
        Self::Scalar(b.into())
    }

    /// Use when you need a [binary][1] value for DynamoDB.
    ///
    /// See also: [`Scalar::new_binary`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-B
    pub fn new_binary<B>(binary: B) -> Self
    where
        B: Into<Vec<u8>>,
    {
        Self::Scalar(binary.into().into())
    }

    /// Use when you need a [null][1] value for DynamoDB.
    ///
    /// See also: [`Scalar::new_null`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-NULL
    pub fn new_null() -> Self {
        Self::Scalar(Scalar::Null)
    }
}

impl From<Scalar> for Value {
    fn from(value: Scalar) -> Self {
        Self::Scalar(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Scalar::from(value).into()
    }
}

impl From<&String> for Value {
    fn from(value: &String) -> Self {
        Scalar::from(value).into()
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Scalar::from(value).into()
    }
}

impl From<&&str> for Value {
    fn from(value: &&str) -> Self {
        Scalar::from(value).into()
    }
}

impl From<Num> for Value {
    fn from(value: Num) -> Self {
        Scalar::from(value).into()
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Scalar::from(value).into()
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Scalar::from(value).into()
    }
}

impl<const N: usize> From<[u8; N]> for Value {
    fn from(value: [u8; N]) -> Self {
        Scalar::from(value).into()
    }
}

impl From<()> for Value {
    fn from(value: ()) -> Self {
        Scalar::from(value).into()
    }
}

impl FromIterator<u8> for Value {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = u8>,
    {
        Scalar::from_iter(iter).into()
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
