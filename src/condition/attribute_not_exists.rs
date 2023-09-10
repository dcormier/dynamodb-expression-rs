use core::fmt::{self, Write};

use crate::Name;

/// True if the attribute specified by `path` does not exist in the item.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttributeNotExists {
    pub(crate) name: Name,
}

impl fmt::Display for AttributeNotExists {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("attribute_not_exists(")?;
        self.name.fmt(f)?;
        f.write_char(')')
    }
}

impl<T> From<T> for AttributeNotExists
where
    T: Into<Name>,
{
    fn from(name: T) -> Self {
        Self { name: name.into() }
    }
}
