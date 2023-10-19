use core::fmt;

use crate::{path::Path, value::List};

/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.UpdatingListElements>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Append {
    /// The field to set the newly combined list to
    // TODO: Name or Path?
    dst: Path,

    /// The field to get the current list from
    // TODO: Name or Path?
    src: Path,

    /// The value(s) to add to the list to the list
    value: List,

    /// Whether to add the new values to the beginning or end of the source list
    before_or_after: BeforeOrAfter,
}

impl Append {
    /// Sets up an [`Append`] where the source list to get the initial value from,
    /// and the destination field to save the updated list to, are the same. E.g.,
    /// `my_field = list_append(my_field, [1,2,3,4])`
    pub fn new_with_self_to_end<P, L>(field: P, value: L) -> Self
    where
        P: Into<Path>,
        L: Into<List>,
    {
        let field = field.into();

        Self::new_with_source(field.clone(), field, value.into(), BeforeOrAfter::After)
    }
    pub fn new_with_self_to_beginning<P, L>(field: P, value: L) -> Self
    where
        P: Into<Path>,
        L: Into<List>,
    {
        let field = field.into();

        Self::new_with_source(field.clone(), field, value.into(), BeforeOrAfter::Before)
    }

    pub fn new_with_source<D, S, L>(
        destination: D,
        source: S,
        value: L,
        before_or_after: BeforeOrAfter,
    ) -> Self
    where
        D: Into<Path>,
        S: Into<Path>,
        L: Into<List>,
    {
        Self {
            dst: destination.into(),
            src: source.into(),
            value: value.into(),
            before_or_after,
        }
    }
}

impl fmt::Display for Append {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            dst,
            src,
            value,
            before_or_after,
        } = self;

        write!(f, "{dst} = list_append(")?;

        match before_or_after {
            BeforeOrAfter::Before => write!(f, "{value}, {src})"),
            BeforeOrAfter::After => write!(f, "{src}, {value})"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BeforeOrAfter {
    Before,
    After,
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_str_eq;

    use super::{Append, BeforeOrAfter};

    #[test]
    fn display() {
        let append = Append::new_with_source("foo", "bar", ["a", "b"], BeforeOrAfter::After);
        assert_str_eq!(r#"foo = list_append(bar, ["a", "b"])"#, append.to_string());

        let append = Append::new_with_source("foo", "bar", ["a", "b"], BeforeOrAfter::Before);
        assert_str_eq!(r#"foo = list_append(["a", "b"], bar)"#, append.to_string());

        let append = Append::new_with_self_to_end("foo", ["a", "b"]);
        assert_str_eq!(r#"foo = list_append(foo, ["a", "b"])"#, append.to_string());

        let append = Append::new_with_self_to_beginning("foo", ["a", "b"]);
        assert_str_eq!(r#"foo = list_append(["a", "b"], foo)"#, append.to_string());
    }
}
