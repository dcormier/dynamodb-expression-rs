use core::fmt;
use std::collections::BTreeSet;

use aws_sdk_dynamodb::types::AttributeValue;

/// Represents a [DynamoDB string set][1].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringSet(BTreeSet<String>);

impl StringSet {
    /// Creates a value to use as a [DynamoDB string set][1].
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
    pub fn new<T>(set: T) -> Self
    where
        T: Into<StringSet>,
    {
        set.into()
    }

    // Intentionally not using `impl From<StringSet> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    pub(super) fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::Ss(self.0.into_iter().collect())
    }
}

impl<T> FromIterator<T> for StringSet
where
    T: Into<String>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

impl<I, T> From<I> for StringSet
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    fn from(values: I) -> Self {
        Self::from_iter(values)
    }
}

impl fmt::Display for StringSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}
