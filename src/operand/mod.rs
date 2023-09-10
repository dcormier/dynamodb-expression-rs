mod size;

pub use size::Size;

use core::fmt;

use crate::{
    condition::{Between, Comparator, Comparison, Condition, In},
    Name, ScalarValue,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operand {
    Name(Name),
    Value(ScalarValue),
    Condition(Box<Condition>),
    Size(Size),
}

impl Operand {
    /// Compare two values.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
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
        match self {
            Operand::Name(operand) => operand.fmt(f),
            Operand::Value(operand) => operand.fmt(f),
            Operand::Condition(operand) => operand.fmt(f),
            Operand::Size(operand) => operand.fmt(f),
        }
    }
}

impl From<Name> for Operand {
    fn from(name: Name) -> Self {
        Self::Name(name)
    }
}

impl From<ScalarValue> for Operand {
    fn from(value: ScalarValue) -> Self {
        Self::Value(value)
    }
}

impl From<Condition> for Operand {
    fn from(condition: Condition) -> Self {
        Self::Condition(Box::from(condition))
    }
}

impl From<Box<Condition>> for Operand {
    fn from(condition: Box<Condition>) -> Self {
        Self::Condition(condition)
    }
}

impl From<Size> for Operand {
    fn from(size: Size) -> Self {
        Self::Size(size)
    }
}
