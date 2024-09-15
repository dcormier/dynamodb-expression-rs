//! Types related to [DynamoDB update expressions][1]. For more, see [`Update`].
//!
//! [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html

mod add;
mod delete;
mod remove;
mod set;

use core::fmt;

pub use self::{
    add::{Add, AddAction, AddValue},
    delete::{Delete, DeleteAction},
    remove::Remove,
    set::{
        if_not_exists, list_append, math, Assign, IfNotExists, ListAppend, Math, Set, SetAction,
    },
};

/// Represents a [DynamoDB update expression][1].
///
/// See also: [`Expression`], [`Set`], [`Remove`], [`Add`], [`Delete`]
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use dynamodb_expression::{update::Update, value::StringSet, Expression, Path};
/// # use pretty_assertions::assert_eq;
///
/// let update = Update::from("foo".parse::<Path>()?.math().add(7));
/// assert_eq!("SET foo = foo + 7", update.to_string());
///
/// let update = Update::from("foo".parse::<Path>()?.if_not_exists().set("a value"));
/// assert_eq!(
///     r#"SET foo = if_not_exists(foo, "a value")"#,
///     update.to_string()
/// );
///
/// let update = Update::from("foo".parse::<Path>()?.remove());
/// assert_eq!(r#"REMOVE foo"#, update.to_string());
///
/// let update = Update::from(
///     "foo"
///         .parse::<Path>()?
///         .add(StringSet::from(["a value", "another value"])),
/// );
/// assert_eq!(
///     r#"ADD foo ["a value", "another value"]"#,
///     update.to_string()
/// );
///
/// let update = Update::from("foo".parse::<Path>()?.delete(StringSet::from(["a value"])));
/// assert_eq!(r#"DELETE foo ["a value"]"#, update.to_string());
///
/// // To use an `Update`, build an `Expression`.
/// let expression = Expression::builder().with_update(update).build();
/// # _ = expression;
/// #
/// # Ok(())
/// # }
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html
/// [`Expression`]: crate::Expression
#[must_use = "Use in a DynamoDB expression with `Expression::builder().with_update(update)`"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update {
    pub(crate) set: Option<Set>,
    pub(crate) remove: Option<Remove>,
    pub(crate) add: Option<Add>,
    pub(crate) delete: Option<Delete>,
}

impl Update {
    /// A new update expression for a [`Set`] statement.
    pub fn new_set<T>(set: T) -> Self
    where
        T: Into<Set>,
    {
        set.into().into()
    }

    /// A new update expression for a [`Remove`] statement.
    pub fn new_remove<T>(remove: T) -> Self
    where
        T: Into<Remove>,
    {
        remove.into().into()
    }

    /// A new update expression for an [`Add`] statement.
    pub fn new_add<T>(add: T) -> Self
    where
        T: Into<Add>,
    {
        add.into().into()
    }

    /// A new update expression for a [`Delete`] statement.
    pub fn new_delete<T>(delete: T) -> Self
    where
        T: Into<Delete>,
    {
        delete.into().into()
    }

    /// Combine this [`Update`] statement with another.
    pub fn and<T>(mut self, other: T) -> Self
    where
        T: Into<Update>,
    {
        let other = other.into();

        if let Some(mut other) = other.set {
            if let Some(current) = &mut self.set {
                current.actions.append(&mut other.actions);
            } else {
                self.set = Some(other);
            }
        }

        if let Some(mut other) = other.remove {
            if let Some(current) = &mut self.remove {
                current.paths.append(&mut other.paths);
            } else {
                self.remove = Some(other);
            }
        }

        if let Some(mut other) = other.add {
            if let Some(current) = &mut self.add {
                current.actions.append(&mut other.actions);
            } else {
                self.add = Some(other);
            }
        }

        if let Some(mut other) = other.delete {
            if let Some(current) = &mut self.delete {
                current.actions.append(&mut other.actions);
            } else {
                self.delete = Some(other);
            }
        }

        self
    }
}

impl fmt::Display for Update {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        if let Some(set) = &self.set {
            first = false;
            set.fmt(f)?;
        }

        if let Some(remove) = &self.remove {
            if first {
                first = false;
            } else {
                f.write_str(" ")?;
            }

            remove.fmt(f)?;
        }

        if let Some(add) = &self.add {
            if first {
                first = false;
            } else {
                f.write_str(" ")?;
            }

            add.fmt(f)?;
        }

        if let Some(delete) = &self.delete {
            if !first {
                f.write_str(" ")?;
            }

            delete.fmt(f)?;
        }

        Ok(())
    }
}

impl From<Set> for Update {
    fn from(value: Set) -> Self {
        Self {
            set: Some(value),
            remove: None,
            add: None,
            delete: None,
        }
    }
}

impl From<SetAction> for Update {
    fn from(value: SetAction) -> Self {
        Self {
            set: Some(value.into()),
            remove: None,
            add: None,
            delete: None,
        }
    }
}

impl From<Assign> for Update {
    fn from(value: Assign) -> Self {
        Self {
            set: Some(value.into()),
            remove: None,
            add: None,
            delete: None,
        }
    }
}

impl From<Math> for Update {
    fn from(value: Math) -> Self {
        Self {
            set: Some(value.into()),
            remove: None,
            add: None,
            delete: None,
        }
    }
}

impl From<ListAppend> for Update {
    fn from(value: ListAppend) -> Self {
        Self {
            set: Some(value.into()),
            remove: None,
            add: None,
            delete: None,
        }
    }
}

impl From<IfNotExists> for Update {
    fn from(value: IfNotExists) -> Self {
        Self {
            set: Some(value.into()),
            remove: None,
            add: None,
            delete: None,
        }
    }
}

impl From<Remove> for Update {
    fn from(value: Remove) -> Self {
        Self {
            set: None,
            remove: Some(value),
            add: None,
            delete: None,
        }
    }
}

impl From<Add> for Update {
    fn from(value: Add) -> Self {
        Self {
            set: None,
            remove: None,
            add: Some(value),
            delete: None,
        }
    }
}

impl From<AddAction> for Update {
    fn from(value: AddAction) -> Self {
        Self {
            set: None,
            remove: None,
            add: Some(value.into()),
            delete: None,
        }
    }
}

impl From<Delete> for Update {
    fn from(value: Delete) -> Self {
        Self {
            set: None,
            remove: None,
            add: None,
            delete: Some(value),
        }
    }
}

impl From<DeleteAction> for Update {
    fn from(value: DeleteAction) -> Self {
        Self {
            set: None,
            remove: None,
            add: None,
            delete: Some(value.into()),
        }
    }
}

#[cfg(test)]
mod examples {
    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        use crate::{update::Update, value::StringSet, Expression, Path};
        use pretty_assertions::assert_eq;

        let update = Update::from("foo".parse::<Path>()?.math().add(7));
        assert_eq!("SET foo = foo + 7", update.to_string());

        let update = Update::from("foo".parse::<Path>()?.if_not_exists().set("a value"));
        assert_eq!(
            r#"SET foo = if_not_exists(foo, "a value")"#,
            update.to_string()
        );

        let update = Update::from("foo".parse::<Path>()?.remove());
        assert_eq!(r#"REMOVE foo"#, update.to_string());

        let update = Update::from(
            "foo"
                .parse::<Path>()?
                .add(StringSet::from(["a value", "another value"])),
        );
        assert_eq!(
            r#"ADD foo ["a value", "another value"]"#,
            update.to_string()
        );

        let update = Update::from("foo".parse::<Path>()?.delete(StringSet::from(["a value"])));
        assert_eq!(r#"DELETE foo ["a value"]"#, update.to_string());

        // To use an `Update`, build an `Expression`.
        let expression = Expression::builder().with_update(update).build();
        _ = expression;

        Ok(())
    }
}
