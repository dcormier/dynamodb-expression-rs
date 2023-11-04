use core::fmt;

use crate::{
    path::Path,
    value::{List, ValueOrRef},
};

/// Represents an update expression to [append elements to a list][1].
///
/// # Examples
///
/// ```
/// use dynamodb_expression::{Num, path::Name, Path, update::ListAppend};
/// # use pretty_assertions::assert_eq;
///
/// let list_append = ListAppend::builder(Name::new("foo")).list([7, 8, 9].map(Num::new));
/// assert_eq!("foo = list_append(foo, [7, 8, 9])", list_append.to_string());
///
/// let list_append_2 = Path::new_name("foo").list_append().list([7, 8, 9].map(Num::new));
/// assert_eq!(list_append, list_append_2);
/// ```
///
/// If you want to add the new values to the _beginning_ of the list instead,
/// use the [`.before()`] method.
/// ```
/// use dynamodb_expression::{Num, Path, update::ListAppend};
/// # use pretty_assertions::assert_eq;
///
/// let list_append = Path::new_name("foo").list_append().before().list([1, 2, 3].map(Num::new));
/// assert_eq!("foo = list_append([1, 2, 3], foo)", list_append.to_string());
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.UpdatingListElements
/// [`.before()`]: Builder::before
#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
enum Order {
    /// Put the new elements before the existing elements.
    Before,

    /// Add the new elements after the existing elements.
    #[default]
    After,
}

/// Builds an [`ListAppend`] instance. Create an instance of this by using [`ListAppend::builder`].
#[must_use = "Consume the `Builder` using its `.list()` method"]
#[derive(Debug, Clone)]
pub struct Builder {
    dst: Path,
    src: Option<Path>,
    order: Option<Order>,
}

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
    /// Builds the [`ListAppend`] instance.
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

    use crate::path::Name;

    use super::ListAppend;

    #[test]
    fn display() {
        let append = ListAppend::builder(Name::from("foo"))
            .src(Name::from("bar"))
            .after()
            .list(["a", "b"]);
        assert_str_eq!(r#"foo = list_append(bar, ["a", "b"])"#, append.to_string());

        let append = ListAppend::builder(Name::from("foo"))
            .src(Name::from("bar"))
            .list(["a", "b"]);
        assert_str_eq!(r#"foo = list_append(bar, ["a", "b"])"#, append.to_string());

        let append = ListAppend::builder(Name::from("foo"))
            .src(Name::from("bar"))
            .before()
            .list(["a", "b"]);
        assert_str_eq!(r#"foo = list_append(["a", "b"], bar)"#, append.to_string());

        let append = ListAppend::builder(Name::from("foo"))
            .after()
            .list(["a", "b"]);
        assert_str_eq!(r#"foo = list_append(foo, ["a", "b"])"#, append.to_string());

        let append = ListAppend::builder(Name::from("foo")).list(["a", "b"]);
        assert_str_eq!(r#"foo = list_append(foo, ["a", "b"])"#, append.to_string());

        let append = ListAppend::builder(Name::from("foo"))
            .before()
            .list(["a", "b"]);
        assert_str_eq!(r#"foo = list_append(["a", "b"], foo)"#, append.to_string());
    }
}
