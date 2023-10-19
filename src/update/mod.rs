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
