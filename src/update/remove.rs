use core::fmt;

use crate::path::{Indexes, Name, Path};

/// For use an in an [`Update`](crate::update::Update) expression to
/// [remove attributes from an item][1], or [elements from a list][2].
///
/// # Examples
///
/// ```
/// use dynamodb_expression::{path::{Name, Path}, update::{Remove, Update}};
///
/// let update = Remove::name("foo");
/// assert_eq!(r#"REMOVE foo"#, update.to_string());
///
/// let update = Remove::indexed_field("foo", [8]);
/// assert_eq!(r#"REMOVE foo[8]"#, update.to_string());
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Remove {
    // Path is correct here.
    // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.REMOVE.RemovingListElements
    pub(crate) paths: Vec<Path>,
}

impl Remove {
    /// Remove the specified top-level element.
    ///
    /// See also: [`Name`]
    pub fn name<T>(name: T) -> Self
    where
        T: Into<Name>,
    {
        Self {
            paths: vec![name.into().into()],
        }
    }

    /// Constructs a [`Remove`] for an indexed field element of a document path.
    /// For example, `foo[3]` or `foo[7][4]`. If you have a attribute name with
    /// no indexes, you can pass an empty collection, or use [`Remove::name`].
    ///
    /// `indexes` here can be an array, slice, `Vec` of, or single `usize`.
    ///
    /// See also: [`IndexedField`], [`Path::indexed_field`]
    ///
    /// [`Remove::name`]: Self::name
    /// [`IndexedField`]: crate::path::IndexedField
    pub fn indexed_field<N, I>(name: N, indexes: I) -> Self
    where
        N: Into<Name>,
        I: Indexes,
    {
        Self {
            paths: vec![Path::indexed_field(name, indexes)],
        }
    }
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

impl From<Remove> for Vec<Path> {
    fn from(remove: Remove) -> Self {
        remove.paths
    }
}
