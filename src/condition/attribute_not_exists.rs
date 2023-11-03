use core::fmt;

use crate::path::Path;

/// True if the attribute specified by `path` does not exist in the item.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttributeNotExists {
    // `Path` is correct here
    // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Syntax
    pub(crate) path: Path,
}

impl fmt::Display for AttributeNotExists {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "attribute_not_exists({})", self.path)
    }
}

impl<T> From<T> for AttributeNotExists
where
    T: Into<Path>,
{
    fn from(name: T) -> Self {
        Self { path: name.into() }
    }
}
