use core::fmt::{self, Write};

use crate::{
    condition::{Between, Comparator, Comparison, In},
    operand::{Name, Operand},
};

/// Returns a number representing an attribute's size.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Size {
    pub(crate) name: Name,
}

impl Size {
    pub fn between<L, U>(self, lower: L, upper: U) -> Between
    where
        L: Into<Operand>,
        U: Into<Operand>,
    {
        Between {
            op: Operand::Size(self),
            lower: lower.into(),
            upper: upper.into(),
        }
    }

    pub fn in_<I, T>(self, items: I) -> In
    where
        I: IntoIterator<Item = T>,
        T: Into<Operand>,
    {
        In::new(Operand::Size(self), items)
    }

    pub fn comparison<R>(self, cmp: Comparator, right: R) -> Comparison
    where
        R: Into<Operand>,
    {
        Comparison {
            left: Operand::from(self),
            cmp,
            right: right.into(),
        }
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("size(")?;
        self.name.fmt(f)?;
        f.write_char(')')
    }
}

impl<T> From<T> for Size
where
    T: Into<Name>,
{
    fn from(name: T) -> Self {
        Self { name: name.into() }
    }
}
