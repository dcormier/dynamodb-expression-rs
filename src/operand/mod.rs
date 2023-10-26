mod size;

pub use size::Size;

use core::fmt;

use crate::{
    condition::{Between, Comparator, Comparison, Condition, In},
    name::Name,
    value::{Ref, Scalar, ValueOrRef},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Operand {
    pub(crate) op: OperandType,
}

impl Operand {
    /// Compare two values.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    // TODO: Operator-specific methods instead of this.
    pub fn comparison<R>(self, cmp: Comparator, right: R) -> Condition
    where
        R: Into<Operand>,
    {
        Condition::Comparison(Comparison {
            left: self,
            cmp,
            right: right.into(),
        })
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
    Name(Name),
    Scalar(ValueOrRef),
    Condition(Box<Condition>),
    Size(Size),
}

impl fmt::Display for OperandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(operand) => operand.fmt(f),
            Self::Scalar(operand) => operand.fmt(f),
            Self::Condition(operand) => operand.fmt(f),
            Self::Size(operand) => operand.fmt(f),
        }
    }
}

impl From<Name> for OperandType {
    fn from(name: Name) -> Self {
        Self::Name(name)
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
