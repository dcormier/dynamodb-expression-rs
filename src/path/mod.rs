use core::fmt::{self, Write};

use super::name::Name;

/// See <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.NestedAttributes>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    pub path: Vec<Element>,
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        self.path.iter().try_for_each(|elem| {
            if first {
                first = false;
            } else {
                f.write_char('.')?;
            }

            elem.fmt(f)
        })
    }
}

impl<T> From<T> for Path
where
    T: Into<Element>,
{
    fn from(value: T) -> Self {
        Path {
            path: vec![value.into()],
        }
    }
}

impl<T> FromIterator<T> for Path
where
    T: Into<Element>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            path: iter.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Element {
    Name(Name),
    FieldIndex(FieldIndex),
}

impl Element {
    pub fn name<N>(name: N) -> Self
    where
        N: Into<Name>,
    {
        Self::Name(name.into())
    }

    pub fn indexed<N, I>(name: N, indexes: I) -> Self
    where
        N: Into<Name>,
        I: Indexes,
    {
        Self::FieldIndex(FieldIndex {
            name: name.into(),
            indexes: indexes.into_indexes(),
        })
    }
}

impl<T> From<T> for Element
where
    T: Into<Name>,
{
    fn from(name: T) -> Self {
        Self::Name(name.into())
    }
}

impl From<FieldIndex> for Element {
    fn from(value: FieldIndex) -> Self {
        Self::FieldIndex(value)
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Name(name) => name.fmt(f),
            Element::FieldIndex(field_index) => field_index.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldIndex {
    pub(crate) name: Name,
    indexes: Vec<u32>,
}

impl fmt::Display for FieldIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)?;
        self.indexes
            .iter()
            .try_for_each(|index| write!(f, "[{}]", index))
    }
}

impl<N, P> From<(N, P)> for FieldIndex
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
    use pretty_assertions::assert_str_eq;

    use super::{Element, Path};

    #[test]
    fn display_name() {
        let path = Element::name("foo");
        assert_str_eq!("foo", path.to_string());
    }

    #[test]
    fn display_indexed() {
        let path = Element::indexed("foo", 42);
        assert_str_eq!("foo[42]", path.to_string());

        let path = Element::indexed("foo", [42]);
        assert_str_eq!("foo[42]", path.to_string());

        let path = Element::indexed("foo", &([42, 37, 9])[..]);
        assert_str_eq!("foo[42][37][9]", path.to_string());
    }

    #[test]
    fn display_path() {
        let path: Path = ["foo", "bar"].into_iter().collect();
        assert_str_eq!("foo.bar", path.to_string());

        let path = Path::from_iter([Element::name("foo"), Element::indexed("bar", 42)]);
        assert_str_eq!("foo.bar[42]", path.to_string());

        // TODO: I'm not sure this is a legal path based on these examples:
        //       https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.NestedElements.DocumentPathExamples
        //       Test whether it's valid and remove this comment or handle it appropriately.
        let path = Path::from_iter([Element::indexed("foo", 42), Element::name("bar")]);
        assert_str_eq!("foo[42].bar", path.to_string());
    }
}
