//! Types related to [DynamoDB update expressions][1]. For more, see [`Update`].
//!
//! [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html

pub mod add;
pub mod delete;
pub mod remove;
pub mod set;

use core::fmt;

pub use self::{
    add::Add,
    delete::Delete,
    remove::Remove,
    set::{Assign, IfNotExists, ListAppend, Math, Set, SetAction},
};

/// Represents a [DynamoDB update expression][1].
///
/// # Examples
///
/// ```
/// use core::str::FromStr;
///
/// use dynamodb_expression::{
///     update::{Remove, Update},
///     Path,
/// };
/// use itertools::Itertools;
/// # use pretty_assertions::assert_eq;
///
/// let update = Update::from(Path::name("foo").math().add(7));
/// assert_eq!("SET foo = foo + 7", update.to_string());
///
/// let update = Update::from(Path::name("foo").if_not_exists().value("a value"));
/// assert_eq!(
///     r#"SET foo = if_not_exists(foo, "a value")"#,
///     update.to_string()
/// );
///
/// let update = Update::from(Remove::name("foo"));
/// assert_eq!(r#"REMOVE foo"#, update.to_string());
///
/// let update = Update::from("foo[3].bar[0]".parse::<Path>().unwrap().remove());
/// assert_eq!(r#"REMOVE foo[3].bar[0]"#, update.to_string());
///
/// let update = Update::remove(
///     ["foo", "bar", "baz"]
///         .into_iter()
///         .map(Path::from_str)
///         .try_collect::<_, Remove, _>()
///         .unwrap(),
/// );
/// assert_eq!(r#"REMOVE foo, bar, baz"#, update.to_string());
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Update {
    Set(Set),
    Remove(Remove),
    Add(Add),
    Delete(Delete),
}

impl Update {
    /// A new update expression for a [`Set`] statement.
    pub fn set<T>(set: T) -> Self
    where
        T: Into<Set>,
    {
        set.into().into()
    }

    /// A new update expression for a [`Remove`] statement.
    pub fn remove<T>(remove: T) -> Self
    where
        T: Into<Remove>,
    {
        remove.into().into()
    }

    /// A new update expression for an [`Add`] statement.
    pub fn add<T>(add: T) -> Self
    where
        T: Into<Add>,
    {
        add.into().into()
    }

    /// A new update expression for a [`Delete`] statement.
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

impl From<SetAction> for Update {
    fn from(value: SetAction) -> Self {
        Self::Set(value.into())
    }
}

impl From<Assign> for Update {
    fn from(value: Assign) -> Self {
        Self::Set(value.into())
    }
}

impl From<Math> for Update {
    fn from(value: Math) -> Self {
        Self::Set(value.into())
    }
}

impl From<ListAppend> for Update {
    fn from(value: ListAppend) -> Self {
        Self::Set(value.into())
    }
}

impl From<IfNotExists> for Update {
    fn from(value: IfNotExists) -> Self {
        Self::Set(value.into())
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

#[cfg(test)]
mod test {
    #[test]
    #[ignore = "This is just to help with formatting the example for `Update`"]
    fn example() {
        use core::str::FromStr;

        use crate::{
            update::{Remove, Update},
            Path,
        };
        use itertools::Itertools;
        use pretty_assertions::assert_eq;

        let update = Update::from(Path::name("foo").math().add(7));
        assert_eq!("SET foo = foo + 7", update.to_string());

        let update = Update::from(Path::name("foo").if_not_exists().value("a value"));
        assert_eq!(
            r#"SET foo = if_not_exists(foo, "a value")"#,
            update.to_string()
        );

        let update = Update::from(Remove::name("foo"));
        assert_eq!(r#"REMOVE foo"#, update.to_string());

        let update = Update::from("foo[3].bar[0]".parse::<Path>().unwrap().remove());
        assert_eq!(r#"REMOVE foo[3].bar[0]"#, update.to_string());

        let update = Update::remove(
            ["foo", "bar", "baz"]
                .into_iter()
                .map(Path::from_str)
                .try_collect::<_, Remove, _>()
                .unwrap(),
        );
        assert_eq!(r#"REMOVE foo, bar, baz"#, update.to_string());
    }
}
