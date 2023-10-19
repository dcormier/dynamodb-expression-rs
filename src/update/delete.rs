use core::fmt;

use crate::{
    name::Name,
    value::{Set, Value},
};

// func Delete(name NameBuilder, value ValueBuilder) UpdateBuilder
// func (ub UpdateBuilder) Delete(name NameBuilder, value ValueBuilder) UpdateBuilder

/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.DELETE>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Delete {
    // TODO: Name or Path?
    path: Name,
    subset: Value,
}

impl Delete {
    pub fn new<P, S>(path: P, subset: S) -> Self
    where
        P: Into<Name>,
        S: Into<Set>,
    {
        Self {
            path: path.into(),
            subset: subset.into().into(),
        }
    }
}

impl fmt::Display for Delete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DELETE {} {}", self.path, self.subset)
    }
}
