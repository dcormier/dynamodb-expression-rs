use core::fmt::{self, Write};

use crate::operand::Operand;

/// Represents a [DynamoDB comparison operation][1] for use in a [`Condition`].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators
/// [`Condition`]: crate::condition::Condition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comparison {
    pub(crate) left: Operand,
    pub(crate) cmp: Comparator,
    pub(crate) right: Operand,
}

impl fmt::Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.left.fmt(f)?;
        f.write_char(' ')?;
        self.cmp.fmt(f)?;
        f.write_char(' ')?;
        self.right.fmt(f)
    }
}

/**
[DynamoDB comparison operators](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)

```no-compile
comparator ::=
    =
    | <>
    | <
    | <=
    | >
    | >=
*/
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Comparator {
    /// Equal (`=`)
    Eq,
    /// Not equal (`<>`)
    Ne,
    /// Less than (`<`)
    Lt,
    /// Less than or equal (`<=`)
    Le,
    /// Greater than (`>`)
    Gt,
    /// Greater than or equal (`>=`)
    Ge,
}

impl Comparator {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Eq => "=",
            Self::Ne => "<>",
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
        }
    }
}

impl fmt::Display for Comparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Check if the two [values][1] or [paths][2] are equal.
///
/// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
///
/// [1]: [crate::value]
/// [2]: [crate::path::Path]
pub fn equal<L, R>(left: L, right: R) -> Comparison
where
    L: Into<Operand>,
    R: Into<Operand>,
{
    Comparison {
        left: left.into(),
        cmp: Comparator::Eq,
        right: right.into(),
    }
}

/// Check if the two [values][1] or [paths][2] are not equal.
///
/// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
///
/// [1]: [crate::value]
/// [2]: [crate::path::Path]
pub fn not_equal<L, R>(left: L, right: R) -> Comparison
where
    L: Into<Operand>,
    R: Into<Operand>,
{
    Comparison {
        left: left.into(),
        cmp: Comparator::Ne,
        right: right.into(),
    }
}

/// Check if a [value][1] or [`Path`] is greater than another.
///
/// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
///
/// [1]: [crate::value]
/// [`Path`]: crate::path::Path
pub fn greater_than<L, R>(left: L, right: R) -> Comparison
where
    L: Into<Operand>,
    R: Into<Operand>,
{
    Comparison {
        left: left.into(),
        cmp: Comparator::Gt,
        right: right.into(),
    }
}

/// Check if a [value][1] or [`Path`] is greater than or equal to another.
///
/// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
///
/// [1]: [crate::value]
/// [`Path`]: crate::path::Path
pub fn greater_than_or_equal<L, R>(left: L, right: R) -> Comparison
where
    L: Into<Operand>,
    R: Into<Operand>,
{
    Comparison {
        left: left.into(),
        cmp: Comparator::Ge,
        right: right.into(),
    }
}

/// Check if a [value][1] or [`Path`] is less than another.
///
/// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
///
/// [1]: [crate::value]
/// [`Path`]: crate::path::Path
pub fn less_than<L, R>(left: L, right: R) -> Comparison
where
    L: Into<Operand>,
    R: Into<Operand>,
{
    Comparison {
        left: left.into(),
        cmp: Comparator::Lt,
        right: right.into(),
    }
}

/// Check if a [value][1] or [`Path`] is less than or equal to another.
///
/// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
///
/// [1]: [crate::value]
/// [`Path`]: crate::path::Path
pub fn less_than_or_equal<L, R>(left: L, right: R) -> Comparison
where
    L: Into<Operand>,
    R: Into<Operand>,
{
    Comparison {
        left: left.into(),
        cmp: Comparator::Le,
        right: right.into(),
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_str_eq;

    use crate::path::Name;

    use super::{Comparator::*, *};

    #[test]
    fn display() {
        assert_str_eq!("=", Eq.to_string());
        assert_str_eq!("<>", Ne.to_string());
        assert_str_eq!("<", Lt.to_string());
        assert_str_eq!("<=", Le.to_string());
        assert_str_eq!(">", Gt.to_string());
        assert_str_eq!(">=", Ge.to_string());
    }

    #[test]
    fn eq() {
        assert_eq!(
            "foo = bar",
            equal(Name::from("foo"), Name::from("bar")).to_string()
        );
    }

    #[test]
    fn ne() {
        assert_eq!(
            "foo <> bar",
            not_equal(Name::from("foo"), Name::from("bar")).to_string()
        );
    }

    #[test]
    fn lt() {
        assert_eq!(
            "foo < bar",
            less_than(Name::from("foo"), Name::from("bar")).to_string()
        );
    }

    #[test]
    fn le() {
        assert_eq!(
            "foo <= bar",
            less_than_or_equal(Name::from("foo"), Name::from("bar")).to_string()
        );
    }

    #[test]
    fn gt() {
        assert_eq!(
            "foo > bar",
            greater_than(Name::from("foo"), Name::from("bar")).to_string()
        );
    }

    #[test]
    fn ge() {
        assert_eq!(
            "foo >= bar",
            greater_than_or_equal(Name::from("foo"), Name::from("bar")).to_string()
        );
    }
}
