use core::fmt;
use std::collections::BTreeSet;

use aws_sdk_dynamodb::types::AttributeValue;

use crate::Num;

/// Represents a [DynamoDB number set][1].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumSet(BTreeSet<Num>);

impl NumSet {
    /// Creates a value to use as a [DynamoDB number set][1].
    ///
    /// See also: [`Set::new_num_set`], [`Value::new_num_set`], [`Num`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use pretty_assertions::assert_eq;
    /// #
    /// use dynamodb_expression::value::{Num, NumSet};
    ///
    /// assert_eq!(
    ///     NumSet::new([Num::from(1), Num::from(2), Num::from(3)]),
    ///     NumSet::new([1, 2, 2, 3]),
    /// );
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
    /// [`Set::new_num_set`]: crate::Set::new_num_set
    /// [`Value::new_num_set`]: crate::Value::new_num_set
    pub fn new<T>(set: T) -> Self
    where
        T: Into<NumSet>,
    {
        set.into()
    }

    // Intentionally not using `impl From<NumSet> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    pub(super) fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::Ns(self.0.into_iter().map(Into::into).collect())
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
        Self(iter.into_iter().map(Into::into).collect())
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
        f.debug_list().entries(self.0.iter().map(DebugNum)).finish()
    }
}

struct DebugNum<'a>(&'a Num);

impl<'a> fmt::Debug for DebugNum<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0.n)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::{Num, NumSet};

    #[test]
    fn test_display() {
        let set = NumSet::new([1, 2, 3]);
        assert_eq!("[1, 2, 3]", set.to_string());
    }

    #[test]
    fn test_mixed_num_types() {
        let set = NumSet::new([
            Num::new(1),
            Num::new_lower_exp(41_000),
            Num::new_upper_exp(2600),
        ]);
        assert_eq!("[1, 2.6E3, 4.1e4]", set.to_string());
    }
}
