use core::fmt;

use crate::{
    path::Path,
    value::{Set, ValueOrRef},
};

// func Delete(name NameBuilder, value ValueBuilder) UpdateBuilder
// func (ub UpdateBuilder) Delete(name NameBuilder, value ValueBuilder) UpdateBuilder

/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.DELETE>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delete {
    // TODO: Name or Path?
    pub(crate) path: Path,
    pub(crate) subset: ValueOrRef,
}

impl Delete {
    pub fn new<P, S>(path: P, subset: S) -> Self
    where
        P: Into<Path>,
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
