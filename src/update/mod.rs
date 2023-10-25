pub mod add;
pub mod delete;
pub mod remove;
pub mod set;

use core::fmt;

pub use self::{add::Add, delete::Delete, remove::Remove, set::Set};

/// See: <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Update {
    Set(Set),
    Remove(Remove),
    Add(Add),
    Delete(Delete),
}

impl Update {
    pub fn set<T>(set: T) -> Self
    where
        T: Into<Set>,
    {
        set.into().into()
    }

    pub fn remove<T>(remove: T) -> Self
    where
        T: Into<Remove>,
    {
        remove.into().into()
    }

    pub fn add<T>(add: T) -> Self
    where
        T: Into<Add>,
    {
        add.into().into()
    }

    pub fn delete<T>(delete: T) -> Self
    where
        T: Into<Delete>,
    {
        delete.into().into()
    }
}

impl fmt::Display for Update {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Update::Set(update) => update.fmt(f),
            Update::Remove(update) => update.fmt(f),
            Update::Add(update) => update.fmt(f),
            Update::Delete(update) => update.fmt(f),
        }
    }
}

impl From<Set> for Update {
    fn from(value: Set) -> Self {
        Self::Set(value)
    }
}

impl From<Remove> for Update {
    fn from(value: Remove) -> Self {
        Self::Remove(value)
    }
}

impl From<Add> for Update {
    fn from(value: Add) -> Self {
        Self::Add(value)
    }
}

impl From<Delete> for Update {
    fn from(value: Delete) -> Self {
        Self::Delete(value)
    }
}
