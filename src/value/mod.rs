//! Types related to values used in [DynamoDB update expressions][1]. For more, see [`Update`].
//!
//! [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html
//! [`Update`]: crate::update::Update

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
use std::error::Error;

use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue};
use base64::{engine::general_purpose, Engine as _};
use itertools::Itertools;

/// A DynamoDB value
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Scalar(Scalar),
    Set(Set),
    Map(Map),
    List(List),
}

impl Value {
    /// Use when you need a [string value][1] for DynamoDB.
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

    /// Use when you need a [numeric value][1] for DynamoDB.
    ///
    /// See also: [`Num::new`], [`Scalar::new_num`],
    /// [`Value::new_num_lower_exp`], [`Value::new_num_upper_exp`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-N
    pub fn new_num<N>(value: N) -> Self
    where
        N: ToString + ::num::Num,
    {
        Num::new(value).into()
    }

    /// Use when you need a [numeric value][1] for DynamoDB in exponent form
    /// (with a lowercase `e`).
    ///
    /// See also: [`Num::new_lower_exp`], [`Scalar::new_num_lower_exp`],
    /// [`Value::new_num_upper_exp`], [`Value::new_num`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-N
    pub fn new_num_lower_exp<N>(value: N) -> Self
    where
        N: LowerExp + ::num::Num,
    {
        Num::new_lower_exp(value).into()
    }

    /// Use when you need a [numeric value][1] for DynamoDB in exponent form
    /// (with an uppercase `E`).
    ///
    /// See also: [`Num::new_upper_exp`], [`Scalar::new_num_upper_exp`],
    /// [`Value::new_num_lower_exp`], [`Value::new_num`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-N
    pub fn new_num_upper_exp<N>(value: N) -> Self
    where
        N: UpperExp + ::num::Num,
    {
        Num::new_upper_exp(value).into()
    }

    /// Use when you need a [boolean value][1] for DynamoDB.
    ///
    /// See also: [`Scalar::new_bool`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-BOOL
    pub fn new_bool(b: bool) -> Self {
        b.into()
    }

    /// Use when you need a [binary value][1] for DynamoDB.
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

    /// Use when you need a [null value][1] for DynamoDB.
    ///
    /// See also: [`Scalar::new_null`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_AttributeValue.html#DDB-Type-AttributeValue-NULL
    pub fn new_null() -> Self {
        Scalar::new_null().into()
    }

    /// Creates a value to use as a [DynamoDB string set][1].
    ///
    /// See also: [`StringSet::new`], [`Set::new_string_set`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
    pub fn new_string_set<T>(string_set: T) -> Self
    where
        T: Into<StringSet>,
    {
        string_set.into().into()
    }

    /// Creates a value to use as a [DynamoDB number set][1].
    ///
    /// See also: [`NumSet::new`], [`Set::new_num_set`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
    pub fn new_num_set<T>(num_set: T) -> Self
    where
        T: Into<NumSet>,
    {
        num_set.into().into()
    }

    /// Creates a value to use as a [DynamoDB binary set][1].
    ///
    /// See also: [`BinarySet::new`], [`Set::new_binary_set`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
    pub fn new_binary_set<T>(binary_set: T) -> Self
    where
        T: Into<BinarySet>,
    {
        binary_set.into().into()
    }

    /// Creates a value to use as a [DynamoDB map][1].
    ///
    /// See also: [`Map::new`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.Document.Map
    pub fn new_map<T>(map: T) -> Self
    where
        T: Into<Map>,
    {
        map.into().into()
    }

    /// Creates a value to use as a [DynamoDB list][1].
    ///
    /// See also: [`List::new`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.Document.List
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

impl TryFrom<AttributeValue> for Value {
    type Error = UnknownAttributeValueError;

    /// This will only return an error if a new [`AttributeValue`] variant is
    /// added to the AWS DynamoDB SDK and isn't supported here, yet.
    ///
    /// See: [`UnknownAttributeValueError`], [`AttributeValue::Unknown`]
    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        Ok(match value {
            AttributeValue::B(b) => Scalar::Binary(b.into_inner()).into(),
            AttributeValue::Bool(b) => Scalar::Bool(b).into(),
            AttributeValue::Bs(bs) => {
                BinarySet::from_iter(bs.into_iter().map(Blob::into_inner)).into()
            }
            AttributeValue::L(l) => List::from(
                l.into_iter()
                    .map(Self::try_from)
                    .try_collect::<_, Vec<_>, _>()?,
            )
            .into(),
            AttributeValue::M(m) => Map::from(
                m.into_iter()
                    .map(|(k, v)| Self::try_from(v).map(|v| (k, v)))
                    .try_collect::<_, Vec<_>, _>()?,
            )
            .into(),
            AttributeValue::N(n) => Num { n }.into(),
            AttributeValue::Ns(ns) => NumSet::from_iter(ns.into_iter().map(|n| Num { n })).into(),
            AttributeValue::Null(_null) => Scalar::Null.into(),
            AttributeValue::S(s) => Scalar::String(s).into(),
            AttributeValue::Ss(ss) => StringSet::from(ss).into(),
            _ => return Err(UnknownAttributeValueError(value)),
        })
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

/// An error that may occur when converting an [`AttributeValue`] into a
/// [`Value`] (via `.try_from()`/`.try_into()`). This will only occur if a new
/// `AttributeValue` variant is added to the AWS DynamoDB SDK and isn't
/// supported here, yet.
///
/// The [`AttributeValue`] with the unknown variant is included in this error.
///
/// See: [`AttributeValue::Unknown`]
#[derive(Debug)]
pub struct UnknownAttributeValueError(pub AttributeValue);

impl fmt::Display for UnknownAttributeValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown AttributeValue variant: {:?}", self.0)
    }
}

impl Error for UnknownAttributeValueError {}

/// Produces base64 the way DynamoDB wants it.
pub(crate) fn base64<T>(b: T) -> String
where
    T: AsRef<[u8]>,
{
    general_purpose::STANDARD.encode(b)
}

#[cfg(test)]
mod test {
    use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue};
    use pretty_assertions::assert_eq;

    use crate::value::{BinarySet, List, Map, Num, NumSet, StringSet};

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
    fn from_attribute_value() {
        assert_eq!(
            Value::from(Map::from([
                ("s", Value::from("a string")),
                ("int", Value::from(Num::from(8))),
                ("b", Value::from(b"foo".to_vec())),
                ("null", Value::from(())),
                ("yes", Value::from(true)),
                ("no", Value::from(false)),
                (
                    "list",
                    List::from([
                        Value::from("foo"),
                        Value::from(Num::from(42)),
                        Value::from(()),
                    ])
                    .into(),
                ),
                ("ss", StringSet::from(["foo"]).into()),
                ("ns", NumSet::from([Num::from(42)]).into()),
                (
                    "bs",
                    BinarySet::from([b"foo".to_vec(), b"bar".to_vec(), b"baz".to_vec()]).into()
                ),
            ])),
            Value::try_from(AttributeValue::M(
                [
                    ("s".to_string(), AttributeValue::S("a string".to_string())),
                    ("int".to_string(), AttributeValue::N("8".to_string())),
                    ("b".to_string(), AttributeValue::B(Blob::new(b"foo"))),
                    ("null".to_string(), AttributeValue::Null(true)),
                    ("yes".to_string(), AttributeValue::Bool(true)),
                    ("no".to_string(), AttributeValue::Bool(false)),
                    (
                        "list".to_string(),
                        AttributeValue::L(vec![
                            AttributeValue::S("foo".to_string()),
                            AttributeValue::N("42".to_string()),
                            AttributeValue::Null(true),
                        ]),
                    ),
                    (
                        "ss".to_string(),
                        AttributeValue::Ss(vec!["foo".to_string()])
                    ),
                    ("ns".to_string(), AttributeValue::Ns(vec!["42".to_string()])),
                    (
                        "bs".to_string(),
                        AttributeValue::Bs(vec![
                            Blob::new(b"foo"),
                            Blob::new(b"bar"),
                            Blob::new(b"baz"),
                        ])
                    ),
                ]
                .into_iter()
                .collect(),
            ))
            .expect("Could not convert AttributeValue to Value"),
        );
    }
}
