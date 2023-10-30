use core::fmt;

use crate::path::Path;

/// For use an in an [`Update`](crate::update::Update) expression to
/// [remove attributes from an item][1], or [elements from a list][2].
///
/// # Examples
///
/// ```
/// use dynamodb_expression::{path::{Name, Path}, update::{Remove, Update}};
///
/// let update = Remove::from(Name::from("foo"));
/// assert_eq!(r#"REMOVE foo"#, update.to_string());
///
/// let update = Remove::from("foo[8]".parse::<Path>().unwrap());
/// assert_eq!(r#"REMOVE foo[8]"#, update.to_string());
///
/// let update = Remove::from_iter(["foo", "bar", "baz"].map(Name::from));
/// assert_eq!(r#"REMOVE foo, bar, baz"#, update.to_string());
///
/// let update = ["foo", "bar", "baz"].into_iter().map(Name::from).collect::<Remove>();
/// assert_eq!(r#"REMOVE foo, bar, baz"#, update.to_string());
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.REMOVE
/// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.REMOVE.RemovingListElements
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Remove {
    // Path is correct here.
    // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.REMOVE.RemovingListElements
    pub(crate) paths: Vec<Path>,
}

impl<T> From<T> for Remove
where
    T: Into<Path>,
{
    fn from(path: T) -> Self {
        Self {
            paths: vec![path.into()],
        }
    }
}

impl<T> FromIterator<T> for Remove
where
    T: Into<Path>,
{
    fn from_iter<I>(paths: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            paths: paths.into_iter().map(Into::into).collect(),
        }
    }
}

impl fmt::Display for Remove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("REMOVE ")?;

        let mut first = true;
        self.paths.iter().try_for_each(|name| {
            if first {
                first = false;
            } else {
                f.write_str(", ")?;
            }

            name.fmt(f)
        })
    }
}
