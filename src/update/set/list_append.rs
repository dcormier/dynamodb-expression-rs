use core::fmt::{self, Write};

use crate::{
    path::Path,
    update::{set_remove::SetRemove, Set},
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

    /// Add an additional [`Set`] or [`Remove`] statement to this expression.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let set = "foo"
    ///     .parse::<Path>()?
    ///     .list_append()
    ///     .list([7, 8, 9].map(Num::new))
    ///     .and("bar".parse::<Path>()?.set("a value"));
    /// assert_eq!(r#"SET foo = list_append(foo, [7, 8, 9]), bar = "a value""#, set.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Remove`]: crate::update::Remove
    pub fn and<T>(self, other: T) -> SetRemove
    where
        T: Into<SetRemove>,
    {
        Set::from(self).and(other)
    }
}

impl fmt::Display for ListAppend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.dst.fmt(f)?;
        f.write_str(" = list_append(")?;

        // If no source field is specified, default to using the destination.
        let src = self.src.as_ref().unwrap_or(&self.dst);

        let (first, second): (&dyn fmt::Display, &dyn fmt::Display) = if self.after {
            (src, &self.list)
        } else {
            (&self.list, src)
        };

        first.fmt(f)?;
        f.write_str(", ")?;
        second.fmt(f)?;
        f.write_char(')')
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let list_append = "foo"
    ///     .parse::<Path>()?
    ///     .list_append()
    ///     .src("bar".parse::<Path>()?)
    ///     .list([1, 2, 3].map(Num::new));
    /// assert_eq!("foo = list_append(bar, [1, 2, 3])", list_append.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Compare with what happens without specifying a source [`Path`]:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let list_append = "foo"
    ///     .parse::<Path>()?
    ///     .list_append()
    ///     .list([1, 2, 3].map(Num::new));
    /// assert_eq!("foo = list_append(foo, [1, 2, 3])", list_append.to_string());
    /// #
    /// # Ok(())
    /// # }
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let list_append = "foo".parse::<Path>()?.list_append().after().list([1, 2, 3].map(Num::new));
    /// assert_eq!("foo = list_append(foo, [1, 2, 3])", list_append.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Compare with when [`before`] is used:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let list_append = "foo".parse::<Path>()?.list_append().before().list([1, 2, 3].map(Num::new));
    /// assert_eq!("foo = list_append([1, 2, 3], foo)", list_append.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// The default, with the same behavior as `after`:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let list_append = "foo".parse::<Path>()?.list_append().list([1, 2, 3].map(Num::new));
    /// assert_eq!("foo = list_append(foo, [1, 2, 3])", list_append.to_string());
    /// #
    /// # Ok(())
    /// # }
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let list_append = "foo".parse::<Path>()?.list_append().before().list([1, 2, 3].map(Num::new));
    /// assert_eq!("foo = list_append([1, 2, 3], foo)", list_append.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Compare with when [`after`] is used:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let list_append = "foo".parse::<Path>()?.list_append().after().list([1, 2, 3].map(Num::new));
    /// assert_eq!("foo = list_append(foo, [1, 2, 3])", list_append.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// The default, with the same behavior as [`after`]:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let list_append = "foo".parse::<Path>()?.list_append().list([1, 2, 3].map(Num::new));
    /// assert_eq!("foo = list_append(foo, [1, 2, 3])", list_append.to_string());
    /// #
    /// # Ok(())
    /// # }
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
        update::{set_remove::SetRemove, Assign, Set, SetAction},
        Num, Path,
    };

    use super::ListAppend;

    #[test]
    fn display() -> Result<(), Box<dyn std::error::Error>> {
        let append = ListAppend::builder("foo".parse::<Path>()?)
            .src("bar".parse::<Path>()?)
            .after()
            .list(["a", "b"]);
        assert_eq!(r#"foo = list_append(bar, ["a", "b"])"#, append.to_string());

        let append = ListAppend::builder("foo".parse::<Path>()?)
            .src("bar".parse::<Path>()?)
            .list(["a", "b"]);
        assert_eq!(r#"foo = list_append(bar, ["a", "b"])"#, append.to_string());

        let append = ListAppend::builder("foo".parse::<Path>()?)
            .src("bar".parse::<Path>()?)
            .before()
            .list(["a", "b"]);
        assert_eq!(r#"foo = list_append(["a", "b"], bar)"#, append.to_string());

        let append = ListAppend::builder("foo".parse::<Path>()?)
            .after()
            .list(["a", "b"]);
        assert_eq!(r#"foo = list_append(foo, ["a", "b"])"#, append.to_string());

        let append = ListAppend::builder("foo".parse::<Path>()?).list(["a", "b"]);
        assert_eq!(r#"foo = list_append(foo, ["a", "b"])"#, append.to_string());

        let append = ListAppend::builder("foo".parse::<Path>()?)
            .before()
            .list(["a", "b"]);
        assert_eq!(r#"foo = list_append(["a", "b"], foo)"#, append.to_string());

        Ok(())
    }

    #[test]
    fn and() -> Result<(), Box<dyn std::error::Error>> {
        let list_append = "foo".parse::<Path>()?.list_append().list(["d", "e", "f"]);
        let assign: Assign = "bar".parse::<Path>()?.set(Num::new(8));

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

        let set: Set = [
            SetAction::from(assign),
            SetAction::from("baz".parse::<Path>()?.math().add(1)),
        ]
        .into_iter()
        .collect();
        let combined = list_append.clone().and(set);
        assert_eq!(
            r#"SET foo = list_append(foo, ["d", "e", "f"]), bar = 8, baz = baz + 1"#,
            combined.to_string()
        );

        // Should be able to concatenate a Remove instance

        let combined = list_append.clone().and("quux".parse::<Path>()?.remove());
        assert_eq!(
            r#"SET foo = list_append(foo, ["d", "e", "f"]) REMOVE quux"#,
            combined.to_string()
        );

        // Should be able to concatenate a SetRemove instance

        let combined = list_append.and(SetRemove::from("quux".parse::<Path>()?.remove()));
        assert_eq!(
            r#"SET foo = list_append(foo, ["d", "e", "f"]) REMOVE quux"#,
            combined.to_string()
        );

        Ok(())
    }
}
