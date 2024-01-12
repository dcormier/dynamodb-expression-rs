//! Types related to [DynamoDB update expressions][1]. For more, see [`Update`].
//!
//! [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html

mod add;
mod delete;
mod remove;
mod set;
mod set_remove;

use core::fmt;

pub use self::{
    add::{Add, AddValue},
    delete::Delete,
    remove::Remove,
    set::{
        if_not_exists, list_append, math, Assign, IfNotExists, ListAppend, Math, Set, SetAction,
    },
    set_remove::SetRemove,
};

/// Represents a [DynamoDB update expression][1].
///
/// See also: [`Set`], [`Remove`], [`SetRemove`], [`Add`], [`Delete`]
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use dynamodb_expression::{update::Update, Path};
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
/// #
/// # // TODO: Examples for `Add` and `Delete`.
/// #
/// # Ok(())
/// # }
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html
#[must_use = "Use in a DynamoDB expression with `Expression::builder().with_update(update)`"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Update {
    SetRemove(SetRemove),
    Add(Add),
    Delete(Delete),
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

    /// A new update expression for a [`SetRemove`] statement.
    pub fn new_set_remove<T>(set_remove: T) -> Self
    where
        T: Into<SetRemove>,
    {
        set_remove.into().into()
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
}

impl fmt::Display for Update {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Update::SetRemove(update) => update.fmt(f),
            Update::Add(update) => update.fmt(f),
            Update::Delete(update) => update.fmt(f),
        }
    }
}

impl From<Set> for Update {
    fn from(value: Set) -> Self {
        Self::SetRemove(value.into())
    }
}

impl From<SetAction> for Update {
    fn from(value: SetAction) -> Self {
        Self::SetRemove(value.into())
    }
}

impl From<Assign> for Update {
    fn from(value: Assign) -> Self {
        Self::SetRemove(value.into())
    }
}

impl From<Math> for Update {
    fn from(value: Math) -> Self {
        Self::SetRemove(value.into())
    }
}

impl From<ListAppend> for Update {
    fn from(value: ListAppend) -> Self {
        Self::SetRemove(value.into())
    }
}

impl From<IfNotExists> for Update {
    fn from(value: IfNotExists) -> Self {
        Self::SetRemove(value.into())
    }
}

impl From<Remove> for Update {
    fn from(value: Remove) -> Self {
        Self::SetRemove(value.into())
    }
}

impl From<SetRemove> for Update {
    fn from(value: SetRemove) -> Self {
        Self::SetRemove(value)
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

#[cfg(test)]
mod examples {
    #[test]
    fn example() -> Result<(), Box<dyn std::error::Error>> {
        use crate::{update::Update, Path};
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

        // TODO: Examples for `Add` and `Delete`.

        Ok(())
    }
}
