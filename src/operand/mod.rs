mod size;

pub use size::Size;

use core::fmt;

use crate::{
    condition::{
        equal, greater_than, greater_than_or_equal, less_than, less_than_or_equal, not_equal,
        Between, Condition, In,
    },
    path::Path,
    value::{Ref, Scalar, ValueOrRef},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Operand {
    pub(crate) op: OperandType,
}

impl Operand {
    /// Check if the value of this operand is equal to the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn equal<T>(self, right: T) -> Condition
    where
        T: Into<Operand>,
    {
        equal(self, right).into()
    }

    /// Check if the value of this operand is not equal to the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn not_equal<T>(self, right: T) -> Condition
    where
        T: Into<Operand>,
    {
        not_equal(self, right).into()
    }

    /// Check if the value of this operand is greater than the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn greater_than<T>(self, right: T) -> Condition
    where
        T: Into<Operand>,
    {
        greater_than(self, right).into()
    }

    /// Check if the value of this operand is greater than or equal to the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn greater_than_or_equal<T>(self, right: T) -> Condition
    where
        T: Into<Operand>,
    {
        greater_than_or_equal(self, right).into()
    }

    /// Check if the value of this operand is less than the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn less_than<T>(self, right: T) -> Condition
    where
        T: Into<Operand>,
    {
        less_than(self, right).into()
    }

    /// Check if the value of this operand is less than or equal to the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn less_than_or_equal<T>(self, right: T) -> Condition
    where
        T: Into<Operand>,
    {
        less_than_or_equal(self, right).into()
    }

    /// `self BETWEEN b AND c` - true if `self` is greater than or equal to `b`, and less than or equal to `c`.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn between<L, U>(self, lower: L, upper: U) -> Condition
    where
        L: Into<Operand>,
        U: Into<Operand>,
    {
        Condition::Between(Between {
            op: self,
            lower: lower.into(),
            upper: upper.into(),
        })
    }

    /// `self IN (b[, ..])` — true if `self` is equal to any value in the list.
    ///
    /// The list can contain up to 100 values. It must have at least 1.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn in_<I, T>(self, items: I) -> Condition
    where
        I: IntoIterator<Item = T>,
        T: Into<Operand>,
    {
        Condition::In(In::new(self, items))
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.op.fmt(f)
    }
}

impl<T> From<T> for Operand
where
    T: Into<OperandType>,
{
    fn from(op: T) -> Self {
        Self { op: op.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum OperandType {
    Path(Path),
    Scalar(ValueOrRef),
    Condition(Box<Condition>),
    Size(Size),
}

impl fmt::Display for OperandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Path(operand) => operand.fmt(f),
            Self::Scalar(operand) => operand.fmt(f),
            Self::Condition(operand) => operand.fmt(f),
            Self::Size(operand) => operand.fmt(f),
        }
    }
}

impl From<Path> for OperandType {
    fn from(path: Path) -> Self {
        Self::Path(path)
    }
}

impl From<Scalar> for OperandType {
    fn from(value: Scalar) -> Self {
        Self::Scalar(value.into())
    }
}

impl From<Ref> for OperandType {
    fn from(value: Ref) -> Self {
        Self::Scalar(value.into())
    }
}

impl From<Condition> for OperandType {
    fn from(condition: Condition) -> Self {
        Self::Condition(condition.into())
    }
}

impl From<Box<Condition>> for OperandType {
    fn from(condition: Box<Condition>) -> Self {
        Self::Condition(condition)
    }
}

impl From<Size> for OperandType {
    fn from(size: Size) -> Self {
        Self::Size(size)
    }
}
