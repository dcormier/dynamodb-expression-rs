//! Types related to operands for [DynamoDB condition and filter expressions][1].
//!
//! [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html

mod operand_type;
mod size;

pub(crate) use self::operand_type::OperandType;
pub use self::size::Size;

use core::fmt;

use crate::condition::{
    equal, greater_than, greater_than_or_equal, less_than, less_than_or_equal, not_equal, Between,
    Condition, In,
};

/// Represents a [part of a DynamoDB comparison][1].
///
/// You can use `Operand::from` to construct an instance from any of these:
/// * [`Path`]
/// * [`Element`]
/// * [`Name`]
/// * [`IndexedField`]
/// * [`Scalar`]
/// * [`Ref`]
/// * [`Condition`] (as well as `Box<Condition>`)
/// * [`Size`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html
/// [`Path`]: crate::path::Path
/// [`Element`]: crate::path::Element
/// [`Name`]: crate::path::Name
/// [`IndexedField`]: crate::path::IndexedField
/// [`Scalar`]: crate::value::Scalar
/// [`Ref`]: crate::value::Ref
#[derive(Debug, Clone, PartialEq, Eq)]
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
