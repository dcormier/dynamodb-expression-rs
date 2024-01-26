mod assign;
pub mod if_not_exists;
pub mod list_append;
pub mod math;
mod set_action;

pub use self::assign::Assign;
pub use self::if_not_exists::IfNotExists;
pub use self::list_append::ListAppend;
pub use self::math::Math;
pub use self::set_action::SetAction;

use core::fmt;

use super::Update;

/// Represents a [`SET` statement for an update expression][1]. Most of the time
/// you won't use this directly.
///
/// See also: [`Update`], [`Path::set`], [`Path::if_not_exists`], [`Path::math`]
/// [`Path::list_append`], [`Set::and`]
///
/// # Examples
///
/// ```
/// use dynamodb_expression::{Path, update::Set};
/// # use pretty_assertions::assert_eq;
///
/// let set = Set::from("foo".parse::<Path>().unwrap().math().add(1));
/// assert_eq!("SET foo = foo + 1", set.to_string());
///
/// let set_foo = Set::from("foo".parse::<Path>().unwrap().math().add(1));
/// assert_eq!("SET foo = foo + 1", set_foo.to_string());
///
/// let set_bar = Set::from("bar".parse::<Path>().unwrap().if_not_exists().set("a value"));
/// assert_eq!(r#"SET bar = if_not_exists(bar, "a value")"#, set_bar.to_string());
///
/// let set = set_foo.and(set_bar);
/// assert_eq!(
///     r#"SET foo = foo + 1, bar = if_not_exists(bar, "a value")"#,
///     set.to_string(),
/// );
///
/// let set = Set::from("foo".parse::<Path>().unwrap().list_append().list(["a", "b", "c"]));
/// assert_eq!(r#"SET foo = list_append(foo, ["a", "b", "c"])"#, set.to_string());
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET
/// [`Update`]: crate::update::Update
/// [`Path::set`]: crate::path::Path::set
/// [`Path::if_not_exists`]: crate::path::Path::if_not_exists
/// [`Path::math`]: crate::path::Path::math
/// [`Path::list_append`]: crate::path::Path::list_append
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Set {
    pub(crate) actions: Vec<SetAction>,
}

impl Set {
    /// Add an additional [`Update`] statement to this expression.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// use dynamodb_expression::{Num, Path, update::Set};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let set = "foo"
    ///     .parse::<Path>()?
    ///     .set(Num::new(7))
    ///     .and("bar".parse::<Path>()?.set("a value"))
    ///     .and("baz".parse::<Path>()?.remove());
    /// assert_eq!(r#"SET foo = 7, bar = "a value" REMOVE baz"#, set.to_string());
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

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SET ")?;

        let mut first = true;
        self.actions.iter().try_for_each(|action| {
            if first {
                first = false
            } else {
                f.write_str(", ")?;
            }

            action.fmt(f)
        })
    }
}

impl<T> From<T> for Set
where
    T: Into<SetAction>,
{
    fn from(value: T) -> Self {
        Self {
            actions: vec![value.into()],
        }
    }
}

impl<T> FromIterator<T> for Set
where
    T: Into<SetAction>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            actions: iter.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::{Num, Path};

    use super::{Assign, IfNotExists, ListAppend, Math, Set, SetAction};

    #[test]
    fn from() -> Result<(), Box<dyn std::error::Error>> {
        let assign: Assign = "foo".parse::<Path>()?.set(Num::new(8));
        let if_not_exists: IfNotExists = "bar".parse::<Path>()?.if_not_exists().set(Num::new(7));
        let math: Math = "baz".parse::<Path>()?.math().add(1);
        let list_append: ListAppend = "quux".parse::<Path>()?.list_append().list(["d", "e", "f"]);

        let _set = [
            Set::from(assign.clone()),
            Set::from(if_not_exists),
            Set::from(math),
            Set::from(list_append),
        ];

        let _set = Set::from(SetAction::from(assign));

        Ok(())
    }

    #[test]
    fn and() -> Result<(), Box<dyn std::error::Error>> {
        let assign: Assign = "bar".parse::<Path>()?.set(Num::new(8));
        let set: Set = Set::from("foo".parse::<Path>()?.set("a value"));

        // Should be able to concatenate anything that can be turned into a SetAction.

        let combined = set.clone().and(assign.clone());
        assert_eq!(r#"SET foo = "a value", bar = 8"#, combined.to_string());

        // Should be able to concatenate a SetAction instance.

        let combined = set.clone().and(SetAction::from(assign.clone()));
        assert_eq!(r#"SET foo = "a value", bar = 8"#, combined.to_string());

        // Should be able to concatenate a Set instance

        let set_2: Set = [
            SetAction::from(assign),
            SetAction::from("baz".parse::<Path>()?.if_not_exists().set(Num::new(7))),
        ]
        .into_iter()
        .collect();
        let combined = set.clone().and(set_2);
        assert_eq!(
            r#"SET foo = "a value", bar = 8, baz = if_not_exists(baz, 7)"#,
            combined.to_string()
        );

        // Should be able to concatenate a Remove instance

        let combined = set.clone().and("quux".parse::<Path>()?.remove());
        assert_eq!(r#"SET foo = "a value" REMOVE quux"#, combined.to_string());

        // Should be able to concatenate a SetRemove instance

        let combined = set.and("quux".parse::<Path>()?.remove());
        assert_eq!(r#"SET foo = "a value" REMOVE quux"#, combined.to_string());

        Ok(())
    }
}
