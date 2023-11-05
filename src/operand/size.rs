use core::fmt;

use crate::{
    condition::{
        equal, greater_than, greater_than_or_equal, less_than, less_than_or_equal, not_equal,
        Between, Comparison, In,
    },
    operand::Operand,
    path::Path,
};

/// The [DynamoDB `size` function][1]. Returns a number representing an attributes size.
///
/// See also: [Path::size]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Size {
    // `Path` is correct here
    // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Syntax
    pub(crate) path: Path,
}

impl Size {
    /// Check if the value of this operand is equal to the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn equal<T>(self, right: T) -> Comparison
    where
        T: Into<Operand>,
    {
        equal(self, right)
    }

    /// Check if the value of this operand is not equal to the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn not_equal<T>(self, right: T) -> Comparison
    where
        T: Into<Operand>,
    {
        not_equal(self, right)
    }

    /// Check if the value of this operand is greater than the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn greater_than<T>(self, right: T) -> Comparison
    where
        T: Into<Operand>,
    {
        greater_than(self, right)
    }

    /// Check if the value of this operand is greater than or equal to the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn greater_than_or_equal<T>(self, right: T) -> Comparison
    where
        T: Into<Operand>,
    {
        greater_than_or_equal(self, right)
    }

    /// Check if the value of this operand is less than the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn less_than<T>(self, right: T) -> Comparison
    where
        T: Into<Operand>,
    {
        less_than(self, right)
    }

    /// Check if the value of this operand is less than or equal to the given value.
    ///
    /// [DynamoDB documentation.](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn less_than_or_equal<T>(self, right: T) -> Comparison
    where
        T: Into<Operand>,
    {
        less_than_or_equal(self, right)
    }

    /// The [DynamoDB `BETWEEN` operator][1]. True if `self` is greater than or
    /// equal to `lower`, and less than or equal to `upper`.
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let condition = Path::new_name("foo").size().between(Num::new(512), Num::new(1024));
    /// assert_eq!(r#"size(foo) BETWEEN 512 AND 1024"#, condition.to_string());
    /// ```
    ///
    /// See also: [`Key::between`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators
    /// [`Key::between`]: crate::key::Key::between
    pub fn between<L, U>(self, lower: L, upper: U) -> Between
    where
        L: Into<Operand>,
        U: Into<Operand>,
    {
        Between {
            op: self.into(),
            lower: lower.into(),
            upper: upper.into(),
        }
    }

    /// A [DynamoDB `IN` operation][1]. True if the value at this [`Path`] is equal
    /// to any value in the list.
    ///
    /// The list can contain up to 100 values. It must have at least 1.
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let condition = Path::new_name("foo").size().in_([10, 20, 30, 40, 50].map(Num::new));
    /// assert_eq!(r#"size(foo) IN (10,20,30,40,50)"#, condition.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators
    pub fn in_<I, T>(self, items: I) -> In
    where
        I: IntoIterator<Item = T>,
        T: Into<Operand>,
    {
        In::new(self, items)
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "size({})", self.path)
    }
}

impl<T> From<T> for Size
where
    T: Into<Path>,
{
    fn from(path: T) -> Self {
        Self { path: path.into() }
    }
}
