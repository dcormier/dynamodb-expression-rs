use core::fmt;

use crate::{
    path::Path,
    update::Set,
    value::{List, ValueOrRef},
};

/// Represents an update expression to [append elements to a list][1].
///
/// See also: [`Path::list_append`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.UpdatingListElements
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListAppend {
    /// The field to set the newly combined list to
    pub(crate) dst: Path,

    /// The field to get the current list from
    pub(crate) src: Option<Path>,

    /// The value(s) to add to the list
    pub(crate) list: ValueOrRef,

    /// Whether to add the new values to the beginning or end of the source list
    after: bool,
}

impl ListAppend {
    pub fn builder<T>(dst: T) -> Builder
    where
        T: Into<Path>,
    {
        Builder {
            dst: dst.into(),
            src: None,
            after: true,
        }
    }

    /// Add an additional action to this `SET` statement.
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let set = Path::new_name("foo").list_append().list([7, 8, 9].map(Num::new))
    ///     .and(Path::new_name("bar").assign("a value"));
    /// assert_eq!(r#"SET foo = list_append(foo, [7, 8, 9]), bar = "a value""#, set.to_string());
    /// ```
    pub fn and<T>(self, action: T) -> Set
    where
        T: Into<Set>,
    {
        Set::from(self).and(action)
    }
}

impl fmt::Display for ListAppend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            dst,
            src,
            list,
            after,
        } = self;

        // If no source field is specified, default to using the destination.
        let src = src.as_ref().unwrap_or(dst);

        write!(f, "{dst} = list_append(")?;

        if *after {
            write!(f, "{src}, {list})")
        } else {
            write!(f, "{list}, {src})")
        }
    }
}

/// Builds an [`ListAppend`] instance.
///
/// See also: [`Path::list_append`]
#[must_use = "Consume this `Builder` by using its `.list()` method"]
#[derive(Debug, Clone)]
pub struct Builder {
    dst: Path,
    src: Option<Path>,
    after: bool,
}

impl Builder {
    /// Sets the source [`Path`] to read the initial list from.
    ///
    /// Defaults to the [`Path`] the combined list is being assigned to.
    ///
    /// ```
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let list_append = Path::new_name("foo")
    ///     .list_append()
    ///     .src(Path::new_name("bar"))
    ///     .list([1, 2, 3].map(Num::new));
    /// assert_eq!("foo = list_append(bar, [1, 2, 3])", list_append.to_string());
    /// ```
    ///
    /// Compare with what happens without specifying a source [`Path`]:
    ///
    /// ```
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let list_append = Path::new_name("foo")
    ///     .list_append()
    ///     .list([1, 2, 3].map(Num::new));
    /// assert_eq!("foo = list_append(foo, [1, 2, 3])", list_append.to_string());
    /// ```
    pub fn src<T>(mut self, src: T) -> Self
    where
        T: Into<Path>,
    {
        self.src = Some(src.into());

        self
    }

    /// The new values will be appended to the end of the existing values.
    ///
    /// This is the default.
    ///
    /// ```
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let list_append = Path::new_name("foo").list_append().after().list([1, 2, 3].map(Num::new));
    /// assert_eq!("foo = list_append(foo, [1, 2, 3])", list_append.to_string());
    /// ```
    ///
    /// Compare with when [`before`] is used:
    ///
    /// ```
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let list_append = Path::new_name("foo").list_append().before().list([1, 2, 3].map(Num::new));
    /// assert_eq!("foo = list_append([1, 2, 3], foo)", list_append.to_string());
    /// ```
    ///
    /// The default, with the same behavior as `after`:
    ///
    /// ```
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let list_append = Path::new_name("foo").list_append().list([1, 2, 3].map(Num::new));
    /// assert_eq!("foo = list_append(foo, [1, 2, 3])", list_append.to_string());
    /// ```
    ///
    /// [`before`]: crate::update::set::list_append::Builder::before
    pub fn after(mut self) -> Self {
        self.after = true;

        self
    }

    /// The new values will be placed before the existing values.
    ///
    /// Defaults to appending new values [`after`] existing values.
    ///
    /// ```
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let list_append = Path::new_name("foo").list_append().before().list([1, 2, 3].map(Num::new));
    /// assert_eq!("foo = list_append([1, 2, 3], foo)", list_append.to_string());
    /// ```
    ///
    /// Compare with when [`after`] is used:
    ///
    /// ```
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let list_append = Path::new_name("foo").list_append().after().list([1, 2, 3].map(Num::new));
    /// assert_eq!("foo = list_append(foo, [1, 2, 3])", list_append.to_string());
    /// ```
    ///
    /// The default, with the same behavior as [`after`]:
    ///
    /// ```
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let list_append = Path::new_name("foo").list_append().list([1, 2, 3].map(Num::new));
    /// assert_eq!("foo = list_append(foo, [1, 2, 3])", list_append.to_string());
    /// ```
    ///
    /// [`after`]: crate::update::set::list_append::Builder::after
    pub fn before(mut self) -> Self {
        self.after = false;

        self
    }

    /// Sets the new value(s) to concatenate with the specified field.
    ///
    /// Consumes this [`Builder`] and creates a [`ListAppend`] instance.
    pub fn list<T>(self, list: T) -> ListAppend
    where
        T: Into<List>,
    {
        let Self { dst, src, after } = self;

        ListAppend {
            dst,
            src,
            after,
            list: list.into().into(),
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::{
        path::Name,
        update::{Assign, Set, SetAction},
        Num, Path,
    };

    use super::ListAppend;

    #[test]
    fn display() {
        let append = ListAppend::builder(Name::from("foo"))
            .src(Name::from("bar"))
            .after()
            .list(["a", "b"]);
        assert_eq!(r#"foo = list_append(bar, ["a", "b"])"#, append.to_string());

        let append = ListAppend::builder(Name::from("foo"))
            .src(Name::from("bar"))
            .list(["a", "b"]);
        assert_eq!(r#"foo = list_append(bar, ["a", "b"])"#, append.to_string());

        let append = ListAppend::builder(Name::from("foo"))
            .src(Name::from("bar"))
            .before()
            .list(["a", "b"]);
        assert_eq!(r#"foo = list_append(["a", "b"], bar)"#, append.to_string());

        let append = ListAppend::builder(Name::from("foo"))
            .after()
            .list(["a", "b"]);
        assert_eq!(r#"foo = list_append(foo, ["a", "b"])"#, append.to_string());

        let append = ListAppend::builder(Name::from("foo")).list(["a", "b"]);
        assert_eq!(r#"foo = list_append(foo, ["a", "b"])"#, append.to_string());

        let append = ListAppend::builder(Name::from("foo"))
            .before()
            .list(["a", "b"]);
        assert_eq!(r#"foo = list_append(["a", "b"], foo)"#, append.to_string());
    }

    #[test]
    fn and() {
        let list_append = Path::new_name("foo").list_append().list(["d", "e", "f"]);
        let assign: Assign = Path::new_name("bar").assign(Num::new(8));

        // Should be able to concatenate anything that can be turned into a SetAction.

        let combined = list_append.clone().and(assign.clone());
        assert_eq!(
            r#"SET foo = list_append(foo, ["d", "e", "f"]), bar = 8"#,
            combined.to_string()
        );

        // Should be able to concatenate a SetAction instance.

        let combined = list_append.clone().and(SetAction::from(assign.clone()));
        assert_eq!(
            r#"SET foo = list_append(foo, ["d", "e", "f"]), bar = 8"#,
            combined.to_string()
        );

        // Should be able to concatenate a Set instance

        let set: Set = assign.and(Path::new_name("baz").math().add(1));
        let combined = list_append.and(set);
        assert_eq!(
            r#"SET foo = list_append(foo, ["d", "e", "f"]), bar = 8, baz = baz + 1"#,
            combined.to_string()
        );
    }
}
