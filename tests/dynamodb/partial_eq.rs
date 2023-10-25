// TODO: Pull this out into its own crate.

use std::{
    borrow::Borrow,
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt,
};

// Re-export the AWS SDK we're using
pub use aws_sdk_dynamodb::types::AttributeValue;

use aws_sdk_dynamodb::{
    primitives::Blob,
    types::AttributeValue::{Bs, Ns, Ss, L, M},
};
use itermap::IterMap;
use itertools::Itertools;

use super::{DebugAttributeValue, DebugItem};

/// Provides an equality comparison that accounts for DynamoDB's [unordered sets][1].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
#[derive(Clone)]
pub struct PartialEqItem<T>(pub T)
where
    T: Borrow<HashMap<String, AttributeValue>>;

impl<T> PartialEq for PartialEqItem<T>
where
    T: Borrow<HashMap<String, AttributeValue>>,
{
    fn eq(&self, other: &Self) -> bool {
        fn into_partial_eq(
            item: &HashMap<String, AttributeValue>,
        ) -> BTreeMap<&str, PartialEqAttributeValue<&AttributeValue>> {
            item.iter()
                .map_keys(Borrow::borrow)
                .map_values(PartialEqAttributeValue)
                .collect()
        }

        into_partial_eq(self.0.borrow()) == into_partial_eq(other.0.borrow())
    }
}

impl<T> fmt::Debug for PartialEqItem<T>
where
    T: Borrow<HashMap<String, AttributeValue>>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        DebugItem(self.0.borrow()).fmt(f)
    }
}

/// Provides an equality comparison that accounts for DynamoDB's [unordered sets][1].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
#[derive(Clone)]
pub struct PartialEqAttributeValue<T>(pub T)
where
    T: Borrow<AttributeValue>;

impl<T> PartialEq for PartialEqAttributeValue<T>
where
    T: Borrow<AttributeValue>,
{
    fn eq(&self, other: &Self) -> bool {
        // Compare using a set instead of a vec because DynamoDB set items are
        // unique and unordered. See:
        // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
        fn into_set<'a, I, T>(iter: I) -> BTreeSet<&'a T>
        where
            I: IntoIterator<Item = &'a T> + 'a,
            T: Ord + ?Sized,
        {
            iter.into_iter().collect()
        }

        match (self.0.borrow(), other.0.borrow()) {
            (Ss(this), Ss(other)) => into_set(this) == into_set(other),
            (Ns(this), Ns(other)) => into_set(this) == into_set(other),
            (Bs(this), Bs(other)) => {
                into_set(this.iter().map(Blob::as_ref)) == into_set(other.iter().map(Blob::as_ref))
            }
            (L(this), L(other)) => {
                this.iter().map(PartialEqAttributeValue).collect_vec()
                    == other.iter().map(PartialEqAttributeValue).collect_vec()
            }
            (M(this), M(other)) => PartialEqItem(this) == PartialEqItem(other),
            (this, other) => this == other,
        }
    }
}

impl<T> fmt::Debug for PartialEqAttributeValue<T>
where
    T: Borrow<AttributeValue>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        DebugAttributeValue(self.0.borrow()).fmt(f)
    }
}
