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

/// Represents a [`SET` statement for an update expression][1].
///
/// See also: [`Update`]
///
/// # Examples
///
/// ```
/// use dynamodb_expression::{update::Set, Path};
/// # use pretty_assertions::assert_eq;
///
/// let set_foo = Set::from(Path::new_name("foo").math().add(7));
/// assert_eq!("SET foo = foo + 7", set_foo.to_string());
///
/// let set_bar = Set::from(Path::new_name("bar").if_not_exists().assign("a value"));
/// assert_eq!(r#"SET bar = if_not_exists(bar, "a value")"#, set_bar.to_string());
///
/// let set_foo_and_bar = set_foo.and(set_bar);
/// assert_eq!(
///     r#"SET foo = foo + 7, bar = if_not_exists(bar, "a value")"#,
///     set_foo_and_bar.to_string(),
/// );
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET
/// [`Update`]: crate::update::Update
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Set {
    pub(crate) actions: Vec<SetAction>,
}

impl Set {
    /// Add an additional action to this `SET` statement.
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path, update::Set};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let set = Set::from(Path::new_name("foo").assign(Num::new(7)))
    ///     .and(Path::new_name("bar").assign("a value"));
    /// assert_eq!(r#"SET foo = 7, bar = "a value""#, set.to_string());
    /// ```
    pub fn and<T>(mut self, action: T) -> Self
    where
        T: Into<Set>,
    {
        let mut set = action.into();

        self.actions.append(&mut set.actions);

        self
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
    fn from() {
        let assign: Assign = Path::new_name("foo").assign(Num::new(8));
        let if_not_exists: IfNotExists = Path::new_name("bar").if_not_exists().assign(Num::new(7));
        let math: Math = Path::new_name("baz").math().add(1);
        let list_append: ListAppend = Path::new_name("quux").list_append().list(["d", "e", "f"]);

        let _set = [
            Set::from(assign.clone()),
            Set::from(if_not_exists),
            Set::from(math),
            Set::from(list_append),
        ];

        let _set = Set::from(SetAction::from(assign));
    }

    #[test]
    fn and() {
        let assign: Assign = Path::new_name("bar").assign(Num::new(8));
        let set: Set = Set::from(Path::new_name("foo").assign("a value"));

        // Should be able to concatenate anything that can be turned into a SetAction.

        let combined = set.clone().and(assign.clone());
        assert_eq!(r#"SET foo = "a value", bar = 8"#, combined.to_string());

        // Should be able to concatenate a SetAction instance.

        let combined = set.clone().and(SetAction::from(assign.clone()));
        assert_eq!(r#"SET foo = "a value", bar = 8"#, combined.to_string());

        // Should be able to concatenate a Set instance

        let set_2: Set = assign.and(Path::new_name("baz").if_not_exists().assign(Num::new(7)));
        let combined = set.and(set_2);
        assert_eq!(
            r#"SET foo = "a value", bar = 8, baz = if_not_exists(baz, 7)"#,
            combined.to_string()
        );
    }
}
