use core::{convert::Infallible, fmt, str::FromStr};

/// Represents a DynamoDB [attribute name][1]. This will most commonly be used
/// for [top-level attributes][2].
///
/// Anything that can be turned into a `Name` can be turned into a [`Path`].
///
/// When used in an [`Expression`], attribute `Name`s are
/// automatically handled as [expression attribute names][3], allowing for names
/// that would not otherwise be permitted by DynamoDB. For example, `foo` would
/// become something similar to `#0` in the expression, and the name would be in
/// the `expression_attribute_names`.
///
/// ```
/// use dynamodb_expression::{Name, name};
///
/// // The `name()` function will turn anything that's `Into<String>` into a `Name`.
/// let name: Name = name("foo");
///
/// // A variety of strings can be turned into a `Name`.
/// let name: Name = "foo".into();
/// let name: Name = String::from("foo").into();
/// let name: Name = (&String::from("foo")).into();
/// let name: Name = (&"foo").into();
///
/// // `Name` also implements `FromStr`, so `parse()` can be used.
/// let name: Name = "foo".parse().unwrap();
/// ```
///
/// `Name` and `Path` can be converted between each other.
/// ```
/// use dynamodb_expression::{Name, path::Path};
///
/// // A `Name` can be converted into a `Path`
/// let name = Name::from("foo");
/// let path = Path::from(name);
/// assert_eq!(Path::from("foo"), path);
///
/// // A `Path` consisting of a single, unindexed field can be converted into a `Name`.
/// let path = Path::from("foo");
/// let name = Name::try_from(path).unwrap();
/// assert_eq!(Name::from("foo"), name);
///
/// // If the `Path` has more elements, or has indexes, it cannot be converted
/// // and the original `Path` is returned.
/// let path: Path = "foo[0]".parse().unwrap();
/// let err = Name::try_from(path.clone()).unwrap_err();
/// assert_eq!(path, err);
///
/// let path: Path = "foo.bar".parse().unwrap();
/// let err = Name::try_from(path.clone()).unwrap_err();
/// assert_eq!(path, err);
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html
/// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.TopLevelAttributes
/// [3]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeNames.html
/// [`parse()`]: str::parse
/// [`Expression`]: crate::expression::Expression
/// [`Path`]: crate::path::Path
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name {
    pub(crate) name: String,
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl FromStr for Name {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

// // This would be ideal, but I think trait specialization is needed for this
// // to be workable without causing problems for things that want to do
// // `impl<T: Into<Name>> From<T> for OtherType`.
// impl<T> From<T> for Name
// where
//     T: Into<String>,
// {
//     fn from(name: T) -> Self {
//         Self { name: name.into() }
//     }
// }

impl From<String> for Name {
    fn from(name: String) -> Self {
        Self { name }
    }
}

impl From<&String> for Name {
    fn from(name: &String) -> Self {
        Self::from(name.to_owned())
    }
}

impl From<&str> for Name {
    fn from(name: &str) -> Self {
        Self::from(name.to_owned())
    }
}

impl From<&&str> for Name {
    fn from(name: &&str) -> Self {
        Self::from(name.to_owned())
    }
}

/// A convenience function for creating a [`Name`] instance.
pub fn name<T>(name: T) -> Name
where
    T: Into<String>,
{
    name.into().into()
}
