use core::fmt;

use crate::{
    condition::{
        equal, greater_than, greater_than_or_equal, less_than, less_than_or_equal, not_equal,
        Between, Comparison, In,
    },
    operand::Operand,
    path::Path,
};

/// Returns a number representing an attribute's size.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Size {
    // `Path` is correct here
    // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Syntax
    pub(crate) path: Path,
}

impl Size {
    /// Check if the value of this operand is equal to the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn equal<T>(self, right: T) -> Comparison
    where
        T: Into<Operand>,
    {
        equal(self, right)
    }

    /// Check if the value of this operand is not equal to the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn not_equal<T>(self, right: T) -> Comparison
    where
        T: Into<Operand>,
    {
        not_equal(self, right)
    }

    /// Check if the value of this operand is greater than the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn greater_than<T>(self, right: T) -> Comparison
    where
        T: Into<Operand>,
    {
        greater_than(self, right)
    }

    /// Check if the value of this operand is greater than or equal to the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn greater_than_or_equal<T>(self, right: T) -> Comparison
    where
        T: Into<Operand>,
    {
        greater_than_or_equal(self, right)
    }

    /// Check if the value of this operand is less than the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn less_than<T>(self, right: T) -> Comparison
    where
        T: Into<Operand>,
    {
        less_than(self, right)
    }

    /// Check if the value of this operand is less than or equal to the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn less_than_or_equal<T>(self, right: T) -> Comparison
    where
        T: Into<Operand>,
    {
        less_than_or_equal(self, right)
    }

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
        In::new(self, items)
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
