use core::fmt::{self, Write};

use crate::{
    path::Path,
    value::{self, ValueOrRef},
};

use super::Update;

/// Represents a [`DELETE` statement for an update expression][1], for removing
/// one or more items from a value that is a [set][2].
///
/// Prefer [`Path::delete`] over this.
///
/// See also: [`Update`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.DELETE
/// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
/// [`Update`]: crate::update::Update
#[must_use = "Use in an update expression with `Update::from(delete)`"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delete {
    pub(crate) actions: Vec<DeleteAction>,
}

impl Delete {
    /// Creates a [`Delete`] for the specified [`Path`] and items in that [set][1].
    ///
    /// Prefer [`Path::delete`] over this.
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
    pub fn new<P, S>(path: P, subset: S) -> Self
    where
        P: Into<Path>,
        S: Into<value::Set>,
    {
        Self {
            actions: vec![DeleteAction::new(path, subset)],
        }
    }

    /// Add an additional [`Update`] statement to this `ADD` statement.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// use dynamodb_expression::{value::NumSet, Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let update = "foo"
    ///     .parse::<Path>()?
    ///     .delete(NumSet::from([7]))
    ///     .and("bar".parse::<Path>()?.set("a value"))
    ///     .and("baz".parse::<Path>()?.remove());
    /// assert_eq!(r#"SET bar = "a value" REMOVE baz DELETE foo [7]"#, update.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn and<T>(self, other: T) -> Update
    where
        T: Into<Update>,
    {
        Update::from(self).and(other)
    }
}

impl fmt::Display for Delete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DELETE ")?;

        let mut first = true;
        self.actions.iter().try_for_each(|action| {
            if first {
                first = false;
            } else {
                f.write_str(", ")?;
            }

            action.fmt(f)
        })
    }
}

impl From<DeleteAction> for Delete {
    fn from(action: DeleteAction) -> Self {
        Self {
            actions: vec![action],
        }
    }
}

#[must_use = "Use in an update expression with `Update::from(delete)`"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteAction {
    pub(crate) path: Path,
    pub(crate) subset: ValueOrRef,
}

impl DeleteAction {
    /// Creates a [`DeleteAction`] for the specified [`Path`] and items in that [set][1].
    ///
    /// Prefer [`Path::delete`] over this.
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

    /// Add an additional [`Update`] statement to this `ADD` statement.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// use dynamodb_expression::{update::DeleteAction, value::NumSet, Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let update = DeleteAction::new("foo".parse::<Path>()?, NumSet::from([7]))
    ///     .and("bar".parse::<Path>()?.set("a value"))
    ///     .and("baz".parse::<Path>()?.remove());
    /// assert_eq!(r#"SET bar = "a value" REMOVE baz DELETE foo [7]"#, update.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn and<T>(self, other: T) -> Update
    where
        T: Into<Update>,
    {
        Update::from(self).and(other)
    }
}

impl fmt::Display for DeleteAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.path.fmt(f)?;
        f.write_char(' ')?;
        self.subset.fmt(f)
    }
}
