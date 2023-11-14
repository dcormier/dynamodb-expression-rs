use core::fmt;

/// Represents a DynamoDB [attribute name][1]. This will most commonly be used
/// for [top-level attributes][2].
///
/// This must only be used an attribute name without an index or additional
/// field.
///
/// When used in an [`Expression`], attribute `Name`s are automatically handled
/// as [expression attribute names][3], allowing for names that would not
/// otherwise be permitted by DynamoDB. For example, `foo` would become
/// something similar to `#0` in the expression, and the name would be in the
/// `expression_attribute_names`.
///
/// ```
/// use dynamodb_expression::path::Name;
///
/// // A variety of strings can be turned into a `Name`.
/// let name: Name = "foo".into();
/// let name: Name = String::from("foo").into();
/// let name: Name = (&String::from("foo")).into();
/// let name: Name = (&"foo").into();
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html
/// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.TopLevelAttributes
/// [3]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeNames.html
/// [`Expression`]: crate::expression::Expression
/// [`Path`]: crate::path::Path
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name {
    pub(crate) name: String,
}

impl Name {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self { name: name.into() }
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.name.fmt(f)
    }
}

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

impl From<Name> for String {
    fn from(name: Name) -> Self {
        name.name
    }
}
