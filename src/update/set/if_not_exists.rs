use core::fmt::{self, Write};

use crate::{
    path::Path,
    update::Set,
    value::{Value, ValueOrRef},
};

/// Represents an update expression to [set an attribute if it doesn't exist][1].
///
/// See also: [`Path::if_not_exists`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.PreventingAttributeOverwrites
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfNotExists {
    pub(crate) dst: Path,
    pub(crate) src: Option<Path>,
    pub(crate) value: ValueOrRef,
}

impl IfNotExists {
    pub fn builder<T>(dst: T) -> Builder
    where
        T: Into<Path>,
    {
        Builder {
            dst: dst.into(),
            src: None,
        }
    }

    /// Add an additional action to this `SET` statement.
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path, update::Set};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let set = Path::new_name("foo").if_not_exists().assign(Num::new(7))
    ///     .and(Path::new_name("bar").assign("a value"));
    /// assert_eq!(r#"SET foo = if_not_exists(foo, 7), bar = "a value""#, set.to_string());
    /// ```
    pub fn and<T>(self, action: T) -> Set
    where
        T: Into<Set>,
    {
        Set::from(self).and(action)
    }
}

impl fmt::Display for IfNotExists {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.dst.fmt(f)?;
        f.write_str(" = if_not_exists(")?;
        // If no source field is specified, default to using the destination.
        self.src.as_ref().unwrap_or(&self.dst).fmt(f)?;
        f.write_str(", ")?;
        self.value.fmt(f)?;
        f.write_char(')')
    }
}

/// Builds an [`IfNotExists`] instance. Create an instance of this by using [`IfNotExists::builder`].
///
/// See also: [`Path::if_not_exists`]
#[must_use = "Consume this `Builder` by using its `.value()` method"]
#[derive(Debug, Clone)]
pub struct Builder {
    dst: Path,
    src: Option<Path>,
}

impl Builder {
    /// Sets the source [`Path`] to check for existence.
    ///
    /// Defaults to the destination [`Path`].
    ///
    /// ```
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let if_not_exists = Path::new_name("foo")
    ///     .if_not_exists()
    ///     .src(Path::new_name("bar"))
    ///     .assign(Num::new(42));
    /// assert_eq!("foo = if_not_exists(bar, 42)", if_not_exists.to_string());
    /// ```
    ///
    /// Compare with the default, where the destination [`Path`] is used:
    ///
    /// ```
    /// # use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// let if_not_exists = Path::new_name("foo")
    ///     .if_not_exists()
    ///     .assign(Num::new(42));
    /// assert_eq!("foo = if_not_exists(foo, 42)", if_not_exists.to_string());
    /// ```
    pub fn src<T>(mut self, src: T) -> Self
    where
        T: Into<Path>,
    {
        self.src = Some(src.into());

        self
    }

    /// The value to conditionally set.
    ///
    /// Consumes this [`Builder`] and creates an [`IfNotExists`] instance.
    ///
    /// See also: [`Path::if_not_exists`]
    pub fn assign<T>(self, value: T) -> IfNotExists
    where
        T: Into<Value>,
    {
        let Self { dst, src } = self;

        IfNotExists {
            dst,
            src,
            value: value.into().into(),
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::{
        update::{Assign, Set, SetAction},
        Num, Path,
    };

    use super::IfNotExists;

    #[test]
    fn and() {
        let if_not_exists: IfNotExists = Path::new_name("foo").if_not_exists().assign("a value");
        let assign: Assign = Path::new_name("bar").assign(Num::new(8));

        // Should be able to concatenate anything that can be turned into a SetAction.

        let combined = if_not_exists.clone().and(assign.clone());
        assert_eq!(
            r#"SET foo = if_not_exists(foo, "a value"), bar = 8"#,
            combined.to_string()
        );

        // Should be able to concatenate a SetAction instance.

        let combined = if_not_exists.clone().and(SetAction::from(assign.clone()));
        assert_eq!(
            r#"SET foo = if_not_exists(foo, "a value"), bar = 8"#,
            combined.to_string()
        );

        // Should be able to concatenate a Set instance

        let set: Set = assign.and(Path::new_name("baz").math().add(1));
        let combined = if_not_exists.and(set);
        assert_eq!(
            r#"SET foo = if_not_exists(foo, "a value"), bar = 8, baz = baz + 1"#,
            combined.to_string()
        );
    }
}
