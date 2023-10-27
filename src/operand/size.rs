use core::fmt;

use crate::{
    condition::{Between, Comparator, Comparison, In},
    operand::Operand,
    path::Path,
};

/// Returns a number representing an attribute's size.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Size {
    // `Path` is correct here
    // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Syntax
    pub(crate) path: Path,
}

impl Size {
    pub fn between<L, U>(self, lower: L, upper: U) -> Between
    where
        L: Into<Operand>,
        U: Into<Operand>,
    {
        Between {
            op: self.into(),
            lower: lower.into(),
            upper: upper.into(),
        }
    }

    pub fn in_<I, T>(self, items: I) -> In
    where
        I: IntoIterator<Item = T>,
        T: Into<Operand>,
    {
        In::new(self.into(), items)
    }

    // TODO: Operator-specific methods instead of this.
    pub fn comparison<R>(self, cmp: Comparator, right: R) -> Comparison
    where
        R: Into<Operand>,
    {
        Comparison {
            left: self.into(),
            cmp,
            right: right.into(),
        }
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "size({})", self.path)
    }
}

impl<T> From<T> for Size
where
    T: Into<Path>,
{
    fn from(path: T) -> Self {
        Self { path: path.into() }
    }
}
