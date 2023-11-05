use core::fmt::{self, Write};

use crate::operand::Operand;

/// A [DynamoDB `IN` condition][1].
///
/// See also: [`Path::in_`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators
/// [`Path::in_`]: crate::path::Path::in_
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct In {
    pub(crate) op: Operand,
    pub(crate) items: Vec<Operand>,
}

impl In {
    /// Creates a new [DynamoDB `IN` condition][1]. True if the value from the
    /// [`Operand`] (the `op` parameter) is equal to any value in the list (the
    /// `items` parameter).
    ///
    /// The list can contain up to 100 values. It must have at least 1.
    ///
    /// ```
    /// use dynamodb_expression::{condition::In, operand::Operand, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let condition = In::new(Path::new_name("name"), ["Jack", "Jill"]);
    /// assert_eq!(r#"name IN ("Jack","Jill")"#, condition.to_string());
    /// ```
    ///
    /// See also: [`Path::in_`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators
    /// [`Path::in_`]: crate::path::Path::in_
    pub fn new<O, I, T>(op: O, items: I) -> Self
    where
        O: Into<Operand>,
        I: IntoIterator<Item = T>,
        T: Into<Operand>,
    {
        Self {
            op: op.into(),
            items: items.into_iter().map(Into::into).collect(),
        }
    }
}

impl fmt::Display for In {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.op.fmt(f)?;
        f.write_str(" IN (")?;

        let mut first = true;
        for item in &self.items {
            if first {
                first = false;
            } else {
                f.write_char(',')?;
            }

            item.fmt(f)?;
        }

        f.write_char(')')
    }
}
