use alloc::borrow::Cow;
use core::fmt;

use crate::{
    condition::{
        equal, greater_than, greater_than_or_equal, less_than, less_than_or_equal, Condition,
    },
    operand::Operand,
    Name,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key {
    name: Name,
}

impl Key {
    pub fn begins_with<T>(self, prefix: T) -> KeyCondition
    where
        T: Into<Cow<'static, str>>,
    {
        KeyCondition {
            condition: self.name.begins_with(prefix),
        }
    }

    pub fn between<L, U>(self, lower: L, upper: U) -> KeyCondition
    where
        L: Into<Operand>,
        U: Into<Operand>,
    {
        KeyCondition {
            condition: self.name.between(lower, upper),
        }
    }

    pub fn equal<T>(self, right: T) -> KeyCondition
    where
        T: Into<Operand>,
    {
        KeyCondition {
            condition: equal(self.name, right).into(),
        }
    }

    pub fn greater_than<T>(self, right: T) -> KeyCondition
    where
        T: Into<Operand>,
    {
        KeyCondition {
            condition: greater_than(self.name, right).into(),
        }
    }

    pub fn greater_than_or_equal<T>(self, right: T) -> KeyCondition
    where
        T: Into<Operand>,
    {
        KeyCondition {
            condition: greater_than_or_equal(self.name, right).into(),
        }
    }

    pub fn less_than<T>(self, right: T) -> KeyCondition
    where
        T: Into<Operand>,
    {
        KeyCondition {
            condition: less_than(self.name, right).into(),
        }
    }

    pub fn less_than_or_equal<T>(self, right: T) -> KeyCondition
    where
        T: Into<Operand>,
    {
        KeyCondition {
            condition: less_than_or_equal(self.name, right).into(),
        }
    }
}

impl<T> From<T> for Key
where
    T: Into<Name>,
{
    fn from(name: T) -> Self {
        Self { name: name.into() }
    }
}

pub fn key<T>(name: T) -> Key
where
    T: Into<Name>,
{
    Key::from(name.into())
}

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
