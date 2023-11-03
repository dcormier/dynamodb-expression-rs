use core::fmt;

use crate::condition::Condition;

/// A [logical `OR`][1] operation.
///
/// See: [`Condition`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.LogicalEvaluations

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Or {
    pub(crate) left: Box<Condition>,
    pub(crate) right: Box<Condition>,
}

impl fmt::Display for Or {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} OR {}", self.left, self.right)
    }
}
