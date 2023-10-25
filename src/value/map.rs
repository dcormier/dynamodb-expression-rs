use core::{
    fmt::{self, Write},
    hash,
};

use aws_sdk_dynamodb::types::AttributeValue;
use itermap::IterMap;

use crate::name::Name;

use super::Value;

type MapType<K, V> = std::collections::HashMap<K, V>;
// TODO: Allow this to be configured via feature to switch between HashMap and BTreeMap
// type MapType<K, V> = std::collections::BTreeMap<K, V>;

/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.Document.Map>
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Map {
    map: MapType<Name, Value>,
}

impl Map {
    // Intentionally not using `impl From<ScalarValue> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    pub(super) fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::M(
            self.map
                .into_iter()
                .map_keys(|name| name.name)
                .map_values(Value::into_attribute_value)
                .collect(),
        )
    }
}

impl hash::Hash for Map {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.map.iter().for_each(|(k, v)| {
            k.hash(state);
            v.hash(state);
        })
    }
}

impl fmt::Display for Map {
    // TODO: Test this
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('{')?;

        let mut first = true;
        self.map.iter().try_for_each(|(k, v)| {
            if first {
                first = false;
            } else {
                f.write_str(", ")?;
            }

            write!(f, "{k}: {v}")
        })?;

        f.write_char('}')
    }
}

impl<K, V> FromIterator<(K, V)> for Map
where
    K: Into<Name>,
    V: Into<Value>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        Self {
            map: iter
                .into_iter()
                .map_keys(Into::into)
                .map_values(Into::into)
                .collect(),
        }
    }
}

impl<I, K, V> From<I> for Map
where
    I: IntoIterator<Item = (K, V)>,
    K: Into<Name>,
    V: Into<Value>,
{
    fn from(iter: I) -> Self {
        Self::from_iter(iter)
    }
}
