// TODO: Pull this out into its own crate.

use std::{
    borrow::Borrow,
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt,
};

// Re-export the AWS SDK we're using
pub use aws_sdk_dynamodb::types::AttributeValue;

use aws_sdk_dynamodb::types::AttributeValue::{Bs, Ns, Ss, B, L, M};
use itermap::IterMap;
use itertools::Itertools;

use super::item::base64_encode;

/// Provides a nicer debug view of a DynamoDB item
/// (`HashMap<String, AttributeValue>`), owned or borrowed.
#[derive(Clone, PartialEq)]
pub struct DebugItem<T>(pub T)
where
    T: Borrow<HashMap<String, AttributeValue>>;

impl<T> fmt::Debug for DebugItem<T>
where
    T: Borrow<HashMap<String, AttributeValue>>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .borrow()
            .iter()
            .map_values(DebugAttributeValue)
            .collect::<BTreeMap<_, _>>()
            .fmt(f)
    }
}

/// Provides a nicer debug view of a DynamoDB `AttributeValue`, owned or borrowed.
#[derive(Clone, PartialEq)]
pub struct DebugAttributeValue<T>(pub T)
where
    T: Borrow<AttributeValue>;

impl<T> fmt::Debug for DebugAttributeValue<T>
where
    T: Borrow<AttributeValue>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.borrow() {
            Ss(ss) => f
                .debug_tuple("Ss")
                .field(&ss.iter().collect::<BTreeSet<_>>())
                .finish(),
            Ns(ns) => f
                .debug_tuple("Ns")
                .field(&ns.iter().collect::<BTreeSet<_>>())
                .finish(),
            Bs(bs) => f
                .debug_tuple("Bs")
                .field(
                    &bs.iter()
                        .map(base64_encode)
                        // Using a BTreeSet for consistency when looking at failed comparison.
                        .collect::<BTreeSet<_>>(),
                )
                .finish(),
            // Write bytes as base64 strings
            B(b) => f.debug_tuple("B").field(&base64_encode(b)).finish(),
            // For variants that contain more `AttributeValue`s, write those nicely, too.
            L(l) => f
                .debug_tuple("L")
                .field(&l.iter().map(DebugAttributeValue).collect_vec())
                .finish(),
            M(m) => f
                .debug_tuple("M")
                .field(
                    &m.iter()
                        .map_values(DebugAttributeValue)
                        .collect::<BTreeMap<_, _>>(),
                )
                .finish(),
            // For everything else, write it the say it's normally written.
            _ => self.0.borrow().fmt(f),
        }
    }
}
