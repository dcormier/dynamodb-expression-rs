mod list;
mod map;
mod num;
mod scalar;
mod set;
mod value_or_ref;

pub use list::List;
pub use map::Map;
pub use num::Num;
pub use scalar::Scalar;
pub use set::{BinarySet, NumSet, Set, StringSet};
pub use value_or_ref::{Ref, StringOrRef};

pub(crate) use value_or_ref::ValueOrRef;

use core::fmt::{self, LowerExp, UpperExp};

use aws_sdk_dynamodb::types::AttributeValue;
use base64::{engine::general_purpose, Engine as _};
use itermap::IterMap;

/// A DynamoDB value
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Scalar(Scalar),
    Set(Set),
    Map(Map),
    List(List),
}

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
        value.into().into()
    }

    /// Use when you need a [numeric][1] value for DynamoDB.
    ///
    /// See also:, [`Value::new_num_lower_exp`], [`Value::new_num_upper_exp`],
    /// [`Scalar::new_num`], [`Num::new`]
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
        Num::new(value).into()
    }

    /// Use when you need a [numeric][1] value for DynamoDB in exponent form
    /// (with a lowercase `e`).
    ///
    /// See also:, [`Value::new_num`], [`Value::new_num_upper_exp`],
    /// [`Scalar::new_num_lower_exp`], [`Num::new_lower_exp`]
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
        Num::new_lower_exp(value).into()
    }

    /// Use when you need a [numeric][1] value for DynamoDB in exponent form
    /// (with an uppercase `e`).
    ///
    /// See also:, [`Value::new_num`], [`Value::new_num_lower_exp`],
    /// [`Scalar::new_num_upper_exp`], [`Num::new_upper_exp`]
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
        Num::new_upper_exp(value).into()
    }

    /// Use when you need a [boolean][1] value for DynamoDB.
    ///
    /// See also: [`Scalar::new_bool`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-BOOL
    pub fn new_bool(b: bool) -> Self {
        b.into()
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
        binary.into().into()
    }

    /// Use when you need a [null][1] value for DynamoDB.
    ///
    /// See also: [`Scalar::new_null`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-NULL
    pub fn new_null() -> Self {
        Self::Scalar(Scalar::Null)
    }

    // TODO:
    /// See also: [`Set::new_string_set`], [`StringSet::new`]
    pub fn new_string_set<T>(string_set: T) -> Self
    where
        T: Into<StringSet>,
    {
        string_set.into().into()
    }

    // TODO:
    /// See also: [`Set::new_num_set`], [`NumSet::new`]
    pub fn new_num_set<T>(num_set: T) -> Self
    where
        T: Into<NumSet>,
    {
        num_set.into().into()
    }

    // TODO:
    /// See also: [`Set::new_binary_set`], [`BinarySet::new`]
    pub fn new_binary_set<T>(binary_set: T) -> Self
    where
        T: Into<BinarySet>,
    {
        binary_set.into().into()
    }

    // TODO:
    /// See also: [`Map::new`]
    pub fn new_map<T>(map: T) -> Self
    where
        T: Into<Map>,
    {
        map.into().into()
    }

    // TODO:
    /// See also: [`List::new`]
    pub fn new_list<T>(list: T) -> Self
    where
        T: Into<List>,
    {
        list.into().into()
    }

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

impl From<StringSet> for Value {
    fn from(string_set: StringSet) -> Self {
        Self::Set(string_set.into())
    }
}

impl From<NumSet> for Value {
    fn from(num_set: NumSet) -> Self {
        Self::Set(num_set.into())
    }
}

impl From<BinarySet> for Value {
    fn from(string_set: BinarySet) -> Self {
        Self::Set(string_set.into())
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

impl From<serde_json::Value> for Value {
    /// Converts a [`serde_json::Value`] into a [`Value`].
    ///
    /// A shortcoming of this is that `serde_json::Value` doesn't contain
    /// variants to differentiate between a DynamoDB [list][1] and a [set][2].
    /// The results is that a `serde_json::Value::Array` becomes a `Value::List`.
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.Document.List
    /// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Scalar::Null.into(),
            serde_json::Value::Bool(value) => Scalar::Bool(value).into(),
            serde_json::Value::Number(value) => Num {
                n: value.to_string(),
            }
            .into(),
            serde_json::Value::String(value) => Scalar::String(value).into(),
            serde_json::Value::Array(value) => {
                List::from_iter(value.into_iter().map(Value::from)).into()
            }
            serde_json::Value::Object(value) => {
                Map::from_iter(value.into_iter().map_values(Value::from)).into()
            }
        }
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

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use crate::Num;

    use super::Value;

    #[test]
    fn display() {
        assert_eq!(r#""a""#, Value::new_string("a").to_string());
        assert_eq!(r#"1000"#, Value::new_num(1000).to_string());
        assert_eq!(r#"1e3"#, Value::new_num_lower_exp(1000).to_string());
        assert_eq!(r#"1E3"#, Value::new_num_upper_exp(1000).to_string());
        assert_eq!(r#""YQ==""#, Value::new_binary("a").to_string());
        assert_eq!("true", Value::new_bool(true).to_string());
        assert_eq!("NULL", Value::new_null().to_string());

        // Sets are unordered
        assert_eq!(
            r#"["a", "b", "c"]"#,
            Value::new_string_set(["a", "c", "b"]).to_string()
        );
        assert_eq!(
            r#"[-7, 1e3, 42]"#,
            Value::new_num_set([Num::new_lower_exp(1000), Num::new(42), Num::new(-7)]).to_string()
        );
        assert_eq!(
            r#"["YQ==", "Yg==", "Yw=="]"#,
            Value::new_binary_set([b"a", b"b", b"c"]).to_string()
        );

        assert_eq!(
            r#"[NULL, 8, "a string"]"#,
            Value::new_list([
                Value::new_null(),
                Value::new_num(8),
                Value::new_string("a string")
            ])
            .to_string()
        );

        assert_eq!(
            r#"{n: 8, null: NULL, s: "a string"}"#,
            Value::new_map([
                (String::from("s"), Value::new_string("a string")),
                (String::from("n"), Value::new_num(8)),
                (String::from("null"), Value::new_null()),
            ])
            .to_string()
        );
    }

    #[test]
    fn from_json() {
        assert_eq!(
            Value::new_map([
                (String::from("s"), Value::new_string("a string")),
                (String::from("int"), Value::new_num(8)),
                (String::from("float"), Value::new_num(4.276)),
                (String::from("null"), Value::new_null()),
                (String::from("yes"), Value::new_bool(true)),
                (String::from("no"), Value::new_bool(false)),
                (
                    String::from("list"),
                    Value::new_list([
                        Value::new_string("foo"),
                        Value::new_num(42),
                        Value::new_null(),
                    ])
                ),
            ]),
            Value::from(json!({
                "s": "a string",
                "int": 8,
                "float": 4.276,
                "null": null,
                "yes": true,
                "no": false,
                "list": ["foo", 42, null],
            })),
        );
    }
}
