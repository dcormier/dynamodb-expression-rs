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
    types::AttributeValue::{Bs, Ns, Ss, B, L, M},
};
use itermap::IterMap;
use itertools::Itertools;

use super::item::base64_encode;

/// Provides a nicer debug view of a DynamoDB item
/// (`HashMap<String, AttributeValue>`), owned or borrowed.
#[derive(Clone)]
pub struct DebugItem<T>(pub T)
where
    T: Borrow<HashMap<String, AttributeValue>>;

impl<T: PartialEq> PartialEq for DebugItem<T>
where
    T: Borrow<HashMap<String, AttributeValue>>,
{
    fn eq(&self, other: &Self) -> bool {
        self.0
            .borrow()
            .iter()
            .map_values(DebugAttributeValue)
            .collect::<HashMap<_, _>>()
            == other
                .0
                .borrow()
                .iter()
                .map_values(DebugAttributeValue)
                .collect::<HashMap<_, _>>()
    }
}

impl<T> fmt::Debug for DebugItem<T>
where
    T: Borrow<HashMap<String, AttributeValue>>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .borrow()
            .iter()
            .map_values(DebugAttributeValue)
            // Use a BTreeMap to make the order stable for printing.
            .collect::<BTreeMap<_, _>>()
            .fmt(f)
    }
}

#[derive(Clone)]
pub struct DebugList<I, T>(pub I)
where
    I: Clone + IntoIterator<Item = T>,
    T: Borrow<AttributeValue>;

impl<I: PartialEq, T: PartialEq> PartialEq for DebugList<I, T>
where
    I: Clone + IntoIterator<Item = T>,
    T: Borrow<AttributeValue>,
{
    fn eq(&self, other: &Self) -> bool {
        self.0
            // TODO: Improve or remove this. We don't want to always clone.
            .clone()
            .into_iter()
            .map(DebugAttributeValue)
            .collect_vec()
            == other
                .0
                // TODO: Improve or remove this. We don't want to always clone.
                .clone()
                .into_iter()
                .map(DebugAttributeValue)
                .collect_vec()
    }
}

impl<I, T> fmt::Debug for DebugList<I, T>
where
    I: Clone + IntoIterator<Item = T>,
    T: Borrow<AttributeValue>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            // TODO: Improve or remove this. We don't want to always clone.
            .entries(self.0.clone().into_iter().map(DebugAttributeValue))
            .finish()
    }
}

/// Provides a nicer debug view of a DynamoDB `AttributeValue`, owned or borrowed.
#[derive(Clone)]
pub struct DebugAttributeValue<T>(pub T)
where
    T: Borrow<AttributeValue>;

impl<T: PartialEq> PartialEq for DebugAttributeValue<T>
where
    T: Borrow<AttributeValue>,
{
    fn eq(&self, other: &Self) -> bool {
        /// Compare using a `Set` instead of a `Vec` because [DynamoDB set items
        /// are unique and unordered][1]. I.e., `[1, 2, 3]` and `[2, 3, 1]` are
        /// equivalent sets in DynamoDB. But when represented in a `Vec` (as
        /// are in an [`AttributeValue`]) they are not equivalent.
        ///
        /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
        // TODO: Report this comparison issue as a bug in the AWS DynamoDB SDK.
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
            (L(this), L(other)) => this
                .iter()
                .map(DebugAttributeValue)
                .eq(other.iter().map(DebugAttributeValue)),
            (M(this), M(other)) => DebugItem(this) == DebugItem(other),
            // Other types don't have nested `Vec`s that should be `Set`s and can safely be compared directly
            (this, other) => this == other,
        }
    }
}

impl<T> fmt::Debug for DebugAttributeValue<T>
where
    T: Borrow<AttributeValue>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.borrow() {
            Ss(ss) => f
                .debug_tuple("Ss")
                .field(
                    &ss.iter()
                        // Use a BTreeSet to make the order stable for printing.
                        .collect::<BTreeSet<_>>(),
                )
                .finish(),
            Ns(ns) => f
                .debug_tuple("Ns")
                .field(
                    &ns.iter()
                        // Use a BTreeSet to make the order stable for printing.
                        .collect::<BTreeSet<_>>(),
                )
                .finish(),
            Bs(bs) => f
                .debug_tuple("Bs")
                .field(
                    &bs.iter()
                        // Write bytes as base64 strings
                        .map(base64_encode)
                        // Use a BTreeSet to make the order stable for printing.
                        .collect::<BTreeSet<_>>(),
                )
                .finish(),

            B(b) => f
                .debug_tuple("B")
                // Write bytes as base64 strings
                .field(&base64_encode(b))
                .finish(),
            // For variants that contain more `AttributeValue`s, write those nicely, too.
            L(l) => f
                .debug_tuple("L")
                // .field(&l.iter().map(DebugAttributeValue).collect_vec())
                .field(&DebugList(l))
                .finish(),
            M(m) => f
                .debug_tuple("M")
                .field(
                    &m.iter()
                        .map_values(DebugAttributeValue)
                        // Use a BTreeMap to make the order stable for printing.
                        .collect::<BTreeMap<_, _>>(),
                )
                .finish(),
            // For everything else, write it the way it's normally written.
            _ => self.0.borrow().fmt(f),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue};
    use itermap::IterMap;
    use itertools::Itertools;
    use pretty_assertions::{assert_eq, assert_ne};

    use crate::dynamodb::DebugItem;

    #[test]
    fn partial_eq() {
        let sets: HashMap<String, AttributeValue> = [
            (
                "ss",
                AttributeValue::Ss(["a", "b", "c"].into_iter().map(Into::into).collect()),
            ),
            (
                "ns",
                AttributeValue::Ns(["1", "2", "3"].into_iter().map(Into::into).collect()),
            ),
            (
                "bs",
                AttributeValue::Bs(["foo", "bar", "baz"].into_iter().map(Blob::new).collect()),
            ),
        ]
        .into_iter()
        .map_keys(String::from)
        .collect();

        let mut list = sets.values().cloned().collect_vec();
        list.push(AttributeValue::M(sets.clone()));

        let mut item = sets.clone();
        item.insert("l".into(), AttributeValue::L(list));
        item.insert("m".into(), AttributeValue::M(item.clone()));

        // No longer mutable.
        let item = item;

        // Another item, with items in sets in a different order.
        let item_rot = item
            .clone()
            .into_iter()
            .map_values(rotate_sets)
            .collect::<HashMap<_, _>>();

        // This demonstrates the bug in the AWS SDK. This should be equal, but are not.
        assert_ne!(item, item_rot);

        // This demonstrates that `DebugItem`, `DebugAttributeValue`, and
        // `DebugList` properly implement those equality comparisons.
        assert_eq!(DebugItem(item), DebugItem(item_rot));
    }

    /// Rotates the values of any sets in the [`AttributeValue`] by one, moving
    /// the first value to the end.
    fn rotate_sets(av: AttributeValue) -> AttributeValue {
        fn rotate_vec<T>(mut v: Vec<T>) -> Vec<T> {
            if v.len() < 2 {
                return v;
            }

            // Rotate the first item to the end.
            let first = v.remove(0);
            v.push(first);

            v
        }

        match av {
            AttributeValue::Ss(ss) => AttributeValue::Ss(rotate_vec(ss)),
            AttributeValue::Ns(ns) => AttributeValue::Ns(rotate_vec(ns)),
            AttributeValue::Bs(bs) => AttributeValue::Bs(rotate_vec(bs)),
            AttributeValue::L(l) => AttributeValue::L(l.into_iter().map(rotate_sets).collect()),
            AttributeValue::M(m) => {
                AttributeValue::M(m.into_iter().map_values(rotate_sets).collect())
            }
            // The rest of the variants cannot contain sets
            _ => av,
        }
    }
}
