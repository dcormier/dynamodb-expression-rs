use core::fmt::{self, Write};

use crate::Name;

/// True if the item contains the attribute specified by `path`.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttributeExists {
    pub(crate) name: Name,
}

impl fmt::Display for AttributeExists {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("attribute_exists(")?;
        self.name.fmt(f)?;
        f.write_char(')')
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
