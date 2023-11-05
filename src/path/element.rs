//! DynamoDB document path elements

use core::{
    fmt::{self},
    mem,
    str::FromStr,
};

use super::{Name, PathParseError};

/// Represents a single element of a DynamoDB document [`Path`]. For example,
/// in `foo[3][7].bar[2].baz`, the `Element`s would be `foo[3][7]`, `bar[2]`,
/// and `baz`.
///
/// See also: [`Path`]
///
/// [`Path`]: crate::path::Path
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Element {
    Name(Name),
    IndexedField(IndexedField),
}

impl Element {
    /// An attribute name element of a document path.
    ///
    /// See also: [`Name`], [`Element`], [`Path`]
    ///
    /// [`Path`]: crate::path::Path
    pub fn new_name<T>(name: T) -> Self
    where
        T: Into<Name>,
    {
        Self::Name(name.into())
    }

    /// An indexed field element of a document path. For example, `foo[3]` or
    /// `foo[7][4]`
    ///
    /// `indexes` here can be an array, slice, `Vec` of, or single `usize`.
    /// ```
    /// # use dynamodb_expression::path::Element;
    /// # use pretty_assertions::assert_eq;
    /// #
    /// assert_eq!("foo[3]", Element::new_indexed_field("foo", 3).to_string());
    /// assert_eq!("foo[3]", Element::new_indexed_field("foo", [3]).to_string());
    /// assert_eq!("foo[3]", Element::new_indexed_field("foo", &[3]).to_string());
    /// assert_eq!("foo[3]", Element::new_indexed_field("foo", vec![3]).to_string());
    ///
    /// assert_eq!("foo[7][4]", Element::new_indexed_field("foo", [7, 4]).to_string());
    /// assert_eq!("foo[7][4]", Element::new_indexed_field("foo", &[7, 4]).to_string());
    /// assert_eq!("foo[7][4]", Element::new_indexed_field("foo", vec![7, 4]).to_string());
    ///
    /// assert_eq!("foo", Element::new_indexed_field("foo", []).to_string());
    /// assert_eq!("foo", Element::new_indexed_field("foo", &[]).to_string());
    /// assert_eq!("foo", Element::new_indexed_field("foo", vec![]).to_string());
    /// ```
    ///
    /// See also: [`IndexedField`], [`Path`], [`Path::new_indexed_field`]
    ///
    /// [`Path`]: crate::path::Path
    /// [`Path::new_indexed_field`]: crate::path::Path::new_indexed_field
    pub fn new_indexed_field<N, I>(name: N, indexes: I) -> Self
    where
        N: Into<Name>,
        I: Indexes,
    {
        let indexes = indexes.into_indexes();
        if indexes.is_empty() {
            Self::new_name(name)
        } else {
            Self::IndexedField(IndexedField {
                name: name.into(),
                indexes,
            })
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Name(name) => name.fmt(f),
            Element::IndexedField(field_index) => field_index.fmt(f),
        }
    }
}

impl From<Element> for String {
    fn from(element: Element) -> Self {
        match element {
            Element::Name(name) => name.into(),
            Element::IndexedField(new_indexed_field) => new_indexed_field.to_string(),
        }
    }
}

impl From<IndexedField> for Element {
    fn from(value: IndexedField) -> Self {
        if value.indexes.is_empty() {
            Self::Name(value.name)
        } else {
            Self::IndexedField(value)
        }
    }
}

impl<N, I> From<(N, I)> for Element
where
    N: Into<Name>,
    I: Indexes,
{
    fn from((name, indexes): (N, I)) -> Self {
        Self::new_indexed_field(name, indexes)
    }
}

impl From<Name> for Element {
    fn from(name: Name) -> Self {
        Self::Name(name)
    }
}

// Intentionally not implementing `From` string-types for `Element` to force
// users to intentionally use a `Name` if that's what they want. Should help
// avoid surprises when they have an indexed field, or sub-attribute.

impl FromStr for Element {
    type Err = PathParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut remaining = input;
        let mut name = None;
        let mut indexes = Vec::new();
        while !remaining.is_empty() {
            let open = remaining.find('[');
            let close = remaining.find(']');

            match (open, close) {
                (None, None) => {
                    if name.is_some() {
                        // `bar` in `foo[0]bar`
                        return Err(PathParseError);
                    }

                    // No more braces. Consume the rest of the string.
                    name = Some(mem::take(&mut remaining));
                    break;
                }
                (None, Some(_close)) => return Err(PathParseError),
                (Some(_open), None) => return Err(PathParseError),
                (Some(open), Some(close)) => {
                    if open >= close {
                        // `foo][`
                        return Err(PathParseError);
                    }

                    if name.is_none() {
                        if open > 0 {
                            name = Some(&remaining[..open]);
                        } else {
                            // The string starts with a '['. E.g.:
                            // `[]foo`
                            return Err(PathParseError);
                        }
                    } else if open > 0 {
                        // We've already got the name but we just found another after a closing bracket.
                        // E.g, `bar[0]` in `foo[7]bar[0]`
                        return Err(PathParseError);
                    }

                    // The value between the braces should be a usize.
                    let index: usize = remaining[open + 1..close]
                        .parse()
                        .map_err(|_| PathParseError)?;
                    indexes.push(index);

                    remaining = &remaining[close + 1..];
                }
            }
        }

        Ok(if indexes.is_empty() {
            Self::Name(input.into())
        } else {
            if !remaining.is_empty() {
                // Shouldn't be able to get there.
                // If we do, something above changed and there's a bug.
                return Err(PathParseError);
            }

            let name = name.ok_or(PathParseError)?;

            Self::IndexedField(IndexedField {
                name: name.into(),
                indexes,
            })
        })
    }
}

/// Represents a type of [`Element`] of a DynamoDB document [`Path`] that is a
/// [`Name`] with one or more indexes. For example, in `foo[3][7].bar[2].baz`,
/// the elements `foo[3][7]` and `bar[2]` would both be represented as an
/// `IndexedField`.
///
/// Created via `Element::from`, [`Element::new_indexed_field`], and
/// [`Path::new_indexed_field`].
///
/// [`Path::new_indexed_field`]: crate::path::Path::new_indexed_field
/// [`Path`]: crate::path::Path
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexedField {
    pub(crate) name: Name,
    indexes: Vec<usize>,
}

impl fmt::Display for IndexedField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)?;
        self.indexes
            .iter()
            .try_for_each(|index| write!(f, "[{}]", index))
    }
}

/// Used for [`IndexedField`]. An array, slice, `Vec` of, or single `usize`.
///
/// See also: [`Element::new_indexed_field`], [`Path::new_indexed_field`]
///
/// [`Path::new_indexed_field`]: crate::path::Path::new_indexed_field
pub trait Indexes {
    fn into_indexes(self) -> Vec<usize>;
}

impl Indexes for usize {
    fn into_indexes(self) -> Vec<usize> {
        vec![self]
    }
}

impl Indexes for Vec<usize> {
    fn into_indexes(self) -> Vec<usize> {
        self
    }
}

impl Indexes for &[usize] {
    fn into_indexes(self) -> Vec<usize> {
        self.to_vec()
    }
}

impl<const N: usize> Indexes for [usize; N] {
    fn into_indexes(self) -> Vec<usize> {
        self.to_vec()
    }
}

impl<const N: usize> Indexes for &[usize; N] {
    fn into_indexes(self) -> Vec<usize> {
        self.to_vec()
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::{Num, Path};

    use super::{Element, Name};

    #[test]
    fn display_name() {
        let path = Element::new_name("foo");
        assert_eq!("foo", path.to_string());
    }

    #[test]
    fn display_indexed() {
        // Also tests that `Element::new_indexed_field()` can accept a few different types of input.

        // From a usize
        let path = Element::new_indexed_field("foo", 42);
        assert_eq!("foo[42]", path.to_string());

        // From an array of usize
        let path = Element::new_indexed_field("foo", [42]);
        assert_eq!("foo[42]", path.to_string());

        // From a slice of usize
        let path = Element::new_indexed_field("foo", &([42, 37, 9])[..]);
        assert_eq!("foo[42][37][9]", path.to_string());
    }

    #[test]
    fn display_path() {
        let path: Path = ["foo", "bar"].into_iter().map(Name::from).collect();
        assert_eq!("foo.bar", path.to_string());

        let path = Path::from_iter([
            Element::new_name("foo"),
            Element::new_indexed_field("bar", 42),
        ]);
        assert_eq!("foo.bar[42]", path.to_string());

        // TODO: I'm not sure this is a legal path based on these examples:
        //       https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.NestedElements.DocumentPathExamples
        //       Test whether it's valid and remove this comment or handle it appropriately.
        let path = Path::from_iter([
            Element::new_indexed_field("foo", 42),
            Element::new_name("bar"),
        ]);
        assert_eq!("foo[42].bar", path.to_string());
    }

    #[test]
    fn size() {
        assert_eq!(
            "size(a) = 0",
            "a".parse::<Path>()
                .unwrap()
                .size()
                .equal(Num::new(0))
                .to_string()
        );
    }
}
