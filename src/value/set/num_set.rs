use core::fmt;
use std::collections::BTreeSet;

use aws_sdk_dynamodb::types::AttributeValue;

use crate::Num;

/// A set of unique numeric values for DynamoDB
///
/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumSet(BTreeSet<String>);

impl NumSet {
    /// A set of unique numeric values for DynamoDB
    ///
    /// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes>
    pub fn new<T>(set: T) -> Self
    where
        T: Into<NumSet>,
    {
        set.into()
    }

    // I don't want to make this a public API right now.
    // pub fn insert<T>(&mut self, num: T)
    // where
    //     T: Into<Num>,
    // {
    //     self.0.insert(num.into().into());
    // }

    // Intentionally not using `impl From<NumSet> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    pub(super) fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::Ns(self.0.into_iter().collect())
    }
}

impl<T> FromIterator<T> for NumSet
where
    T: Into<Num>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(iter.into_iter().map(Into::into).map(Into::into).collect())
    }
}

impl<I, T> From<I> for NumSet
where
    I: IntoIterator<Item = T>,
    T: Into<Num>,
{
    fn from(values: I) -> Self {
        Self::from_iter(values)
    }
}

impl fmt::Display for NumSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}
