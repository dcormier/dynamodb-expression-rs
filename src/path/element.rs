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
/// See [`Path`] for more.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Element {
    Name(Name),
    IndexedField(IndexedField),
}

impl Element {
    pub fn name<N>(name: N) -> Self
    where
        N: Into<Name>,
    {
        Self::Name(name.into())
    }

    pub fn indexed_field<N, I>(name: N, indexes: I) -> Self
    where
        N: Into<Name>,
        I: Indexes,
    {
        let indexes = indexes.into_indexes();
        if indexes.is_empty() {
            Self::name(name)
        } else {
            Self::IndexedField(IndexedField {
                name: name.into(),
                indexes: indexes.into_indexes(),
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

                    // The value between the braces should be a u32.
                    let index: u32 = remaining[open + 1..close]
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

impl From<IndexedField> for Element {
    fn from(value: IndexedField) -> Self {
        if value.indexes.is_empty() {
            Self::Name(value.name)
        } else {
            Self::IndexedField(value)
        }
    }
}

impl<N, P> From<(N, P)> for Element
where
    N: Into<Name>,
    P: Indexes,
{
    fn from((name, indexes): (N, P)) -> Self {
        let indexes = indexes.into_indexes();
        if indexes.is_empty() {
            Self::Name(name.into())
        } else {
            Self::IndexedField((name, indexes).into())
        }
    }
}

impl From<Name> for Element {
    fn from(name: Name) -> Self {
        Self::Name(name)
    }
}

impl From<String> for Element {
    fn from(name: String) -> Self {
        Self::name(name)
    }
}

impl From<&String> for Element {
    fn from(name: &String) -> Self {
        Self::name(name)
    }
}

impl From<&str> for Element {
    fn from(name: &str) -> Self {
        Self::name(name)
    }
}

impl From<&&str> for Element {
    fn from(name: &&str) -> Self {
        Self::name(name)
    }
}

/// Represents a type of [`Element`] of a DynamoDB document [`Path`] that is a
/// name with one or more indexes. For example, in `foo[3][7].bar[2].baz`, the
/// elements `foo[3][7]` and `bar[2]` would both be represented as an
/// `IndexedField`.
///
/// See [`Path`] for more.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexedField {
    pub(crate) name: Name,
    indexes: Vec<u32>,
}

impl fmt::Display for IndexedField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)?;
        self.indexes
            .iter()
            .try_for_each(|index| write!(f, "[{}]", index))
    }
}

impl<N, P> From<(N, P)> for IndexedField
where
    N: Into<Name>,
    P: Indexes,
{
    fn from((name, indexes): (N, P)) -> Self {
        Self {
            name: name.into(),
            indexes: indexes.into_indexes(),
        }
    }
}

pub trait Indexes {
    fn into_indexes(self) -> Vec<u32>;
}

impl Indexes for u32 {
    fn into_indexes(self) -> Vec<u32> {
        vec![self]
    }
}

impl Indexes for Vec<u32> {
    fn into_indexes(self) -> Vec<u32> {
        self
    }
}

impl Indexes for &[u32] {
    fn into_indexes(self) -> Vec<u32> {
        self.to_vec()
    }
}

impl<const N: usize> Indexes for [u32; N] {
    fn into_indexes(self) -> Vec<u32> {
        self.to_vec()
    }
}

impl<const N: usize> Indexes for &[u32; N] {
    fn into_indexes(self) -> Vec<u32> {
        self.to_vec()
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::{num_value, Comparator, Path};

    use super::{Element, Name};

    #[test]
    fn display_name() {
        let path = Element::name("foo");
        assert_eq!("foo", path.to_string());
    }

    #[test]
    fn display_indexed() {
        // Also tests that `Element::indexed_field()` can accept a few different types of input.

        // From a u32
        let path = Element::indexed_field("foo", 42);
        assert_eq!("foo[42]", path.to_string());

        // From an array of u32
        let path = Element::indexed_field("foo", [42]);
        assert_eq!("foo[42]", path.to_string());

        // From a slice of u32
        let path = Element::indexed_field("foo", &([42, 37, 9])[..]);
        assert_eq!("foo[42][37][9]", path.to_string());
    }

    #[test]
    fn display_path() {
        let path: Path = ["foo", "bar"].into_iter().map(Name::from).collect();
        assert_eq!("foo.bar", path.to_string());

        let path = Path::from_iter([Element::name("foo"), Element::indexed_field("bar", 42)]);
        assert_eq!("foo.bar[42]", path.to_string());

        // TODO: I'm not sure this is a legal path based on these examples:
        //       https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.NestedElements.DocumentPathExamples
        //       Test whether it's valid and remove this comment or handle it appropriately.
        let path = Path::from_iter([Element::indexed_field("foo", 42), Element::name("bar")]);
        assert_eq!("foo[42].bar", path.to_string());
    }

    #[test]
    fn size() {
        assert_eq!(
            "size(a) = 0",
            "a".parse::<Path>()
                .unwrap()
                .size()
                .comparison(Comparator::Eq, num_value(0))
                .to_string()
        );
    }
}
