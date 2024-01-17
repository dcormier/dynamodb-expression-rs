use core::fmt;

use crate::condition::Condition;

/// Represents a [DynamoDB logical `AND`][1] condition.
///
/// See also: [`Condition::and`]
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use dynamodb_expression::Path;
/// # use pretty_assertions::assert_eq;
///
/// let a = "a".parse::<Path>()?;
/// let b = "b".parse::<Path>()?;
/// let c = "c".parse::<Path>()?;
/// let d = "d".parse::<Path>()?;
///
/// let condition = a.greater_than(b).and(c.less_than(d));
/// assert_eq!("a > b AND c < d", condition.to_string());
/// #
/// # Ok(())
/// # }
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.LogicalEvaluations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct And {
    pub(crate) left: Box<Condition>,
    pub(crate) right: Box<Condition>,
}

impl fmt::Display for And {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.left.fmt(f)?;
        f.write_str(" AND ")?;
        self.right.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::{condition::And, Path};

    #[test]
    fn and() {
        let a = "a".parse::<Path>().unwrap();
        let b = "b".parse::<Path>().unwrap();
        let c = "c".parse::<Path>().unwrap();
        let d = "d".parse::<Path>().unwrap();

        let condition = And {
            left: a.greater_than(b).into(),
            right: c.less_than(d).into(),
        };
        assert_eq!("a > b AND c < d", condition.to_string());
    }
}
