use core::fmt;

use crate::name::Name;

/// True if the item contains the attribute specified by `path`.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttributeExists {
    pub(crate) name: Name,
}

impl fmt::Display for AttributeExists {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "attribute_exists({})", self.name)
    }
}

impl<T> From<T> for AttributeExists
where
    T: Into<Name>,
{
    fn from(name: T) -> Self {
        Self { name: name.into() }
    }
}
