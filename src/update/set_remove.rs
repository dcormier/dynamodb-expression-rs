use core::fmt::{self, Write};

use super::{Assign, IfNotExists, ListAppend, Math, Remove, Set, SetAction};

/// Represents a [`Set`] or [`Remove`] statement for a DynamoDB expression. Most
/// of the time you won't use this directly.
///
/// See: [`Set::and`], [`Remove::and`]
///
/// ```
/// use dynamodb_expression::{Num, Path};
/// # use pretty_assertions::assert_eq;
///
/// let set = Path::new_name("foo").set(Num::new(7)).and(Path::new_name("bar").set("a value"));
/// assert_eq!(r#"SET foo = 7, bar = "a value""#, set.to_string());
///
/// let remove = Path::new_name("baz").remove().and(Path::new_name("quux").remove());
/// assert_eq!("REMOVE baz, quux", remove.to_string());
///
/// let set_remove = set.and(remove);
/// assert_eq!(r#"SET foo = 7, bar = "a value" REMOVE baz, quux"#, set_remove.to_string());
/// ```
#[must_use = "Use in an update expression with `Update::from(set_remove)`"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetRemove {
    pub(crate) set: Option<Set>,
    pub(crate) remove: Option<Remove>,
}

impl SetRemove {
    /// Add an additional [`Set`] or [`Remove`] statement to this expression.
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path, update::SetRemove};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let set = Path::new_name("foo")
    ///     .set(Num::new(7))
    ///     .and(Path::new_name("bar").set("a value"))
    ///     .and(Path::new_name("baz").remove());
    /// assert_eq!(r#"SET foo = 7, bar = "a value" REMOVE baz"#, set.to_string());
    /// ```
    pub fn and<T>(mut self, other: T) -> Self
    where
        T: Into<SetRemove>,
    {
        let other = other.into();

        if let Some(mut other) = other.set {
            self.set = match self.set {
                Some(mut current) => {
                    current.actions.append(&mut other.actions);

                    current
                }
                None => other,
            }
            .into();
        }

        if let Some(mut other) = other.remove {
            self.remove = match self.remove {
                Some(mut current) => {
                    current.paths.append(&mut other.paths);

                    current
                }
                None => other,
            }
            .into();
        }

        self
    }
}

impl fmt::Display for SetRemove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.set, &self.remove) {
            (Some(set), Some(remove)) => {
                set.fmt(f)?;
                f.write_char(' ')?;
                remove.fmt(f)
            }
            (Some(set), None) => set.fmt(f),
            (None, Some(remove)) => remove.fmt(f),
            (None, None) => Ok(()),
        }
    }
}

impl From<Set> for SetRemove {
    fn from(set: Set) -> Self {
        Self {
            set: Some(set),
            remove: None,
        }
    }
}

impl From<SetAction> for SetRemove {
    fn from(value: SetAction) -> Self {
        Set::from(value).into()
    }
}

impl From<Assign> for SetRemove {
    fn from(value: Assign) -> Self {
        Set::from(value).into()
    }
}

impl From<Math> for SetRemove {
    fn from(value: Math) -> Self {
        Set::from(value).into()
    }
}

impl From<ListAppend> for SetRemove {
    fn from(value: ListAppend) -> Self {
        Set::from(value).into()
    }
}

impl From<IfNotExists> for SetRemove {
    fn from(value: IfNotExists) -> Self {
        Set::from(value).into()
    }
}

impl From<Remove> for SetRemove {
    fn from(remove: Remove) -> Self {
        Self {
            set: None,
            remove: Some(remove),
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::{Num, Path};

    use super::*;

    #[test]
    fn display() {
        let set = SetRemove::from(Path::new_name("foo").set("a value"));
        assert_eq!(
            r#"SET foo = "a value""#,
            set.to_string(),
            "Should display only SET"
        );

        let remove = SetRemove::from(Path::new_name("bar").remove());
        assert_eq!(
            "REMOVE bar",
            remove.to_string(),
            "Should display only REMOVE"
        );

        let set_remove = set.clone().and(remove.clone());
        assert_eq!(
            r#"SET foo = "a value" REMOVE bar"#,
            set_remove.to_string(),
            "Should display SET then REMOVE"
        );

        let remove_set = remove.and(set);
        assert_eq!(
            r#"SET foo = "a value" REMOVE bar"#,
            remove_set.to_string(),
            "Should display SET then REMOVE"
        );
    }

    #[test]
    fn test_from_set() {
        let set_remove = SetRemove::from(Set::from(Path::new_name("foo").set("a value")));
        assert_eq!(r#"SET foo = "a value""#, set_remove.to_string());
    }

    #[test]
    fn test_from_set_action() {
        let set_remove = SetRemove::from(SetAction::from(Path::new_name("foo").set("a value")));
        assert_eq!(r#"SET foo = "a value""#, set_remove.to_string());
    }

    #[test]
    fn test_from_assign() {
        let set_remove = SetRemove::from(Assign::new(Path::new_name("foo"), "a value"));
        assert_eq!(r#"SET foo = "a value""#, set_remove.to_string());
    }

    #[test]
    fn test_from_math() {
        let set_remove = SetRemove::from(Math::builder(Path::new_name("foo")).add(Num::from(1)));
        assert_eq!(r#"SET foo = foo + 1"#, set_remove.to_string());
    }

    #[test]
    fn test_from_list_append() {
        let set_remove =
            SetRemove::from(ListAppend::builder(Path::new_name("foo")).list(["a", "b", "c"]));
        assert_eq!(
            r#"SET foo = list_append(foo, ["a", "b", "c"])"#,
            set_remove.to_string()
        );
    }

    #[test]
    fn test_from_if_not_exists() {
        let set_remove =
            SetRemove::from(IfNotExists::builder(Path::new_name("foo")).set("a value"));
        assert_eq!(
            r#"SET foo = if_not_exists(foo, "a value")"#,
            set_remove.to_string()
        );
    }

    #[test]
    fn test_from_remove() {
        let set_remove = SetRemove::from(Path::new_name("foo").remove());
        assert_eq!("REMOVE foo", set_remove.to_string());
    }
}
