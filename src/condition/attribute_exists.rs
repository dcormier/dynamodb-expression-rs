use core::fmt;

use crate::path::Path;
/// The [DynamoDB `attribute_exists` function][1]. True if the item contains
/// the attribute in a specified [`Path`].
///
/// See: [`Path::attribute_exists`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttributeExists {
    // `Path` is correct here
    // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Syntax
    pub(crate) path: Path,
}

impl fmt::Display for AttributeExists {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "attribute_exists({})", self.path)
    }
}

impl<T> From<T> for AttributeExists
where
    T: Into<Path>,
{
    fn from(name: T) -> Self {
        Self { path: name.into() }
    }
}
