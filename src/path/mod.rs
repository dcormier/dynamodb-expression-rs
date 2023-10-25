use core::{
    fmt::{self, Write},
    str::FromStr,
};

use itertools::Itertools;

use super::name::Name;

/// Represents a DynamoDB [document path][1].
///
/// Attribute names are automatically handles as extension attribute names,
/// allowing for names that would not otherwise be permitted by DynamoDB.
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.NestedElements.DocumentPathExamples
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

impl FromStr for Path {
    type Err = PathParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            path: s.split('.').map(str::parse).try_collect()?,
        })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("invalid document path")]
pub struct PathParseError;

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

                    name = Some(remaining);
                    remaining = "";
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
                // Something like `foo[0]bar`
                return Err(PathParseError);
            }

            let name = name.ok_or(PathParseError).map_err(|err| {
                println!("No name found");
                err
            })?;

            Self::FieldIndex(FieldIndex {
                name: name.into(),
                indexes,
            })
        })
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
    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::name::Name;

    use super::{Element, Path};

    #[test]
    fn parse_path() {
        let path: Path = "foo".parse().unwrap();
        assert_eq!(Path::from(Element::from(Name::from("foo"))), path);

        let path: Path = "foo[0]".parse().unwrap();
        assert_eq!(Path::from(Element::indexed("foo", [0])), path);

        let path: Path = "foo[0][3]".parse().unwrap();
        assert_eq!(Path::from(Element::indexed("foo", [0, 3])), path);

        let path: Path = "foo[42][37][9]".parse().unwrap();
        assert_eq!(Path::from(Element::indexed("foo", [42, 37, 9])), path);

        let path: Path = "foo.bar".parse().unwrap();
        assert_eq!(
            Path::from_iter([Element::name("foo"), Element::name("bar")]),
            path
        );

        let path: Path = "foo[42].bar".parse().unwrap();
        assert_eq!(
            Path::from_iter([Element::indexed("foo", 42), Element::name("bar")]),
            path
        );

        let path: Path = "foo.bar[37]".parse().unwrap();
        assert_eq!(
            Path::from_iter([Element::name("foo"), Element::indexed("bar", 37)]),
            path
        );

        let path: Path = "foo[42].bar[37]".parse().unwrap();
        assert_eq!(
            Path::from_iter([Element::indexed("foo", 42), Element::indexed("bar", 37)]),
            path
        );

        let path: Path = "foo[42][7].bar[37]".parse().unwrap();
        assert_eq!(
            Path::from_iter([
                Element::indexed("foo", [42, 7]),
                Element::indexed("bar", 37)
            ]),
            path
        );

        let path: Path = "foo[42].bar[37][9]".parse().unwrap();
        assert_eq!(
            Path::from_iter([
                Element::indexed("foo", 42),
                Element::indexed("bar", [37, 9])
            ]),
            path
        );

        let path: Path = "foo[42][7].bar[37][9]".parse().unwrap();
        assert_eq!(
            Path::from_iter([
                Element::indexed("foo", [42, 7]),
                Element::indexed("bar", [37, 9])
            ]),
            path
        );

        for prefix in ["foo", "foo[0]", "foo.bar", "foo[0]bar", "foo[0]bar[1]"] {
            for bad_index in ["[9", "[]", "][", "[", "]"] {
                let input = format!("{prefix}{bad_index}");

                match input.parse::<Path>() {
                    Ok(path) => {
                        panic!("Should not have parsed invalid input {input:?} into: {path:?}");
                    }
                    Err(_err) => { /* Got the expected error */ }
                }
            }
        }

        // A few other odds and ends
        "foo[0]bar".parse::<Path>().unwrap_err();
        "foo[0]bar[3]".parse::<Path>().unwrap_err();
        "[0]".parse::<Path>().unwrap_err();
    }

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
