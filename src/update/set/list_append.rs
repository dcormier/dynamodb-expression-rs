use core::fmt;

use crate::{
    path::Path,
    value::{List, ValueOrRef},
};

/// Represents an update expression to [append elements to a list][1].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.UpdatingListElements
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ListAppend {
    /// The field to set the newly combined list to
    // TODO: Name or Path?
    pub(crate) dst: Path,

    /// The field to get the current list from
    // TODO: Name or Path?
    pub(crate) src: Option<Path>,

    /// The value(s) to add to the list
    pub(crate) list: ValueOrRef,

    /// Whether to add the new values to the beginning or end of the source list
    order: Order,
}

impl ListAppend {
    pub fn builder<T>(dst: T) -> Builder
    where
        T: Into<Path>,
    {
        Builder {
            dst: dst.into(),
            src: None,
            order: None,
        }
    }
}

impl fmt::Display for ListAppend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            dst,
            src,
            list,
            order,
        } = self;

        // If no source field is specified, default to using the destination.
        let src = src.as_ref().unwrap_or(dst);

        write!(f, "{dst} = list_append(")?;

        match order {
            Order::Before => write!(f, "{list}, {src})"),
            Order::After => write!(f, "{src}, {list})"),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
enum Order {
    /// Put the new elements before the existing elements.
    Before,

    /// Add the new elements after the existing elements.
    #[default]
    After,
}

#[must_use = "Consume the `Builder` using its `.list()` method"]
#[derive(Debug, Clone)]
pub struct Builder {
    dst: Path,
    src: Option<Path>,
    order: Option<Order>,
}

/// Builds an [`Append`] instance. Create an instance of this by using [`Append::builder`].
impl Builder {
    /// Sets the source field to read the initial value from.
    ///
    /// Defaults to the destination field.
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
    pub fn after(mut self) -> Self {
        self.order = Some(Order::After);

        self
    }

    /// The new values will be placed before the existing values.
    ///
    /// Defaults to appending new values after existing values.
    pub fn before(mut self) -> Self {
        self.order = Some(Order::Before);

        self
    }

    /// Sets the new value(s) to concatenate with the specified field.
    ///
    /// Builds the [`Append`] instance.
    pub fn list<T>(self, list: T) -> ListAppend
    where
        T: Into<List>,
    {
        let Self {
            dst,
            src,
            order: op,
        } = self;

        ListAppend {
            dst,
            src,
            order: op.unwrap_or_default(),
            list: list.into().into(),
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_str_eq;

    use super::ListAppend;

    #[test]
    fn display() {
        let append = ListAppend::builder("foo")
            .src("bar")
            .after()
            .list(["a", "b"]);
        assert_str_eq!(r#"foo = list_append(bar, ["a", "b"])"#, append.to_string());

        let append = ListAppend::builder("foo").src("bar").list(["a", "b"]);
        assert_str_eq!(r#"foo = list_append(bar, ["a", "b"])"#, append.to_string());

        let append = ListAppend::builder("foo")
            .src("bar")
            .before()
            .list(["a", "b"]);
        assert_str_eq!(r#"foo = list_append(["a", "b"], bar)"#, append.to_string());

        let append = ListAppend::builder("foo").after().list(["a", "b"]);
        assert_str_eq!(r#"foo = list_append(foo, ["a", "b"])"#, append.to_string());

        let append = ListAppend::builder("foo").list(["a", "b"]);
        assert_str_eq!(r#"foo = list_append(foo, ["a", "b"])"#, append.to_string());

        let append = ListAppend::builder("foo").before().list(["a", "b"]);
        assert_str_eq!(r#"foo = list_append(["a", "b"], foo)"#, append.to_string());
    }
}
