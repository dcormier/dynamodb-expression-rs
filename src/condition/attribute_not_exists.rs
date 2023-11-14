use core::fmt::{self, Write};

use crate::path::Path;

/// The [DynamoDB `attribute_not_exists` function][1]. True if the item does not
/// contain the attribute in a specified [`Path`].
///
/// See also: [`Path::attribute_not_exists`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttributeNotExists {
    // `Path` is correct here
    // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Syntax
    pub(crate) path: Path,
}

impl fmt::Display for AttributeNotExists {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("attribute_not_exists(")?;
        self.path.fmt(f)?;
        f.write_char(')')
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
