use core::fmt;

use crate::condition::Condition;

/// A [logical `AND`][1] operation.
///
/// See: [`Condition`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.LogicalEvaluations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct And {
    pub left: Box<Condition>,
    pub right: Box<Condition>,
}

impl fmt::Display for And {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} AND {}", self.left, self.right)
    }
}
