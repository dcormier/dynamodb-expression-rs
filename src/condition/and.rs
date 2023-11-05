use core::fmt;

use crate::condition::Condition;

/// A [DynamoDB logical `AND`][1] condition.
///
/// See also: [`Condition::and`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.LogicalEvaluations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct And {
    pub(crate) left: Box<Condition>,
    pub(crate) right: Box<Condition>,
}

impl fmt::Display for And {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} AND {}", self.left, self.right)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn and() {
        use crate::{condition::And, Path};
        use pretty_assertions::assert_eq;

        let a = Path::new_name("a");
        let b = Path::new_name("b");
        let c = Path::new_name("c");
        let d = Path::new_name("d");

        let condition = And {
            left: a.greater_than(b).into(),
            right: c.less_than(d).into(),
        };
        assert_eq!("a > b AND c < d", condition.to_string());
    }
}
