use core::fmt::{self, Write};

use crate::{
    path::Path,
    value::{self, ValueOrRef},
};

/// Represents a [`DELETE` statement for an update expression][1], for removing
/// one or more items from a value that is a [set][2].
///
/// See also: [`Path::delete`], [`Update`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.DELETE
/// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
/// [`Update`]: crate::update::Update
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delete {
    pub(crate) path: Path,
    pub(crate) subset: ValueOrRef,
}

impl Delete {
    /// Creates a [`Delete`] for the specified [`Path`] and items in that [set][1].
    ///
    /// See also: [`Path::delete`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
    pub fn new<P, S>(path: P, subset: S) -> Self
    where
        P: Into<Path>,
        S: Into<value::Set>,
    {
        Self {
            path: path.into(),
            subset: subset.into().into(),
        }
    }
}

impl fmt::Display for Delete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DELETE ")?;
        self.path.fmt(f)?;
        f.write_char(' ')?;
        self.subset.fmt(f)
    }
}
