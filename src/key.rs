use core::fmt;

use crate::{
    condition::{
        equal, greater_than, greater_than_or_equal, less_than, less_than_or_equal, Condition,
    },
    operand::Operand,
    path::Path,
    value::StringOrRef,
};

/// Used to build a [key condition expression][1].
///
/// An instance can be constructed using the [`Path::key`] method, or the
/// [`From<Into<Path>>` implementation].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Query.KeyConditionExpressions.html
/// [`From<Into<Path>>` implementation]: Key::from
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key {
    // TODO: Is `Path` the right thing, here?
    //       Probably not. Looks like it should be `Name`
    //
    //       > Furthermore, each primary key attribute must be defined as type string, number, or binary.
    //
    //       https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes
    path: Path,
}

impl Key {
    pub fn begins_with<T>(self, prefix: T) -> KeyCondition
    where
        T: Into<StringOrRef>,
    {
        KeyCondition {
            condition: self.path.begins_with(prefix),
        }
    }

    pub fn between<L, U>(self, lower: L, upper: U) -> KeyCondition
    where
        L: Into<Operand>,
        U: Into<Operand>,
    {
        KeyCondition {
            condition: self.path.between(lower, upper),
        }
    }

    pub fn equal<T>(self, right: T) -> KeyCondition
    where
        T: Into<Operand>,
    {
        KeyCondition {
            condition: equal(self.path, right).into(),
        }
    }

    pub fn greater_than<T>(self, right: T) -> KeyCondition
    where
        T: Into<Operand>,
    {
        KeyCondition {
            condition: greater_than(self.path, right).into(),
        }
    }

    pub fn greater_than_or_equal<T>(self, right: T) -> KeyCondition
    where
        T: Into<Operand>,
    {
        KeyCondition {
            condition: greater_than_or_equal(self.path, right).into(),
        }
    }

    pub fn less_than<T>(self, right: T) -> KeyCondition
    where
        T: Into<Operand>,
    {
        KeyCondition {
            condition: less_than(self.path, right).into(),
        }
    }

    pub fn less_than_or_equal<T>(self, right: T) -> KeyCondition
    where
        T: Into<Operand>,
    {
        KeyCondition {
            condition: less_than_or_equal(self.path, right).into(),
        }
    }
}

impl<T> From<T> for Key
where
    T: Into<Path>,
{
    /// Convert something that implements `Into<Path>` into a [`Key`].
    fn from(path: T) -> Self {
        Self { path: path.into() }
    }
}

/// Represents a DynamoDB [key condition expression].
///
/// See also: [`Key`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Query.KeyConditionExpressions.html
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyCondition {
    pub(crate) condition: Condition,
}

impl KeyCondition {
    pub fn and(self, right: Self) -> Self {
        Self {
            condition: self.condition.and(right.condition),
        }
    }
}

impl fmt::Display for KeyCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.condition.fmt(f)
    }
}

impl From<KeyCondition> for String {
    fn from(key_condition: KeyCondition) -> Self {
        key_condition.condition.into()
    }
}
