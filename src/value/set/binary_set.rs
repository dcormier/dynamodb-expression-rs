use core::fmt;
use std::collections::BTreeSet;

use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue};

use super::base64;

/// A set of unique binary values for DynamoDB
///
/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinarySet(BTreeSet<Vec<u8>>);

impl BinarySet {
    /// A set of unique binary values for DynamoDB
    ///
    /// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes>
    pub fn new<T>(set: T) -> Self
    where
        T: Into<BinarySet>,
    {
        set.into()
    }

    // Intentionally not using `impl From<BinarySet> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    pub(super) fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::Bs(self.0.into_iter().map(Blob::new).collect())
    }
}

impl<T> FromIterator<T> for BinarySet
where
    T: IntoIterator<Item = u8>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(
            iter.into_iter()
                .map(|value| value.into_iter().collect())
                .collect(),
        )
    }
}

// TODO: There's an inconsistency between what turn `Into` a `Binary` and `Into` a `BinarySet`.

impl<I, T> From<I> for BinarySet
where
    I: IntoIterator<Item = T>,
    T: IntoIterator<Item = u8>,
{
    fn from(values: I) -> Self {
        Self::from_iter(values)
    }
}

impl fmt::Display for BinarySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter().map(base64)).finish()
    }
}
