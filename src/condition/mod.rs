//! Types related to conditions for [DynamoDB condition and filter expressions][1].
//!
//! [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html

mod and;
mod attribute_exists;
mod attribute_not_exists;
pub mod attribute_type;
mod begins_with;
mod between;
mod comparison;
mod contains;
mod in_;
mod not;
mod or;
mod parenthetical;

pub use and::And;
pub use attribute_exists::AttributeExists;
pub use attribute_not_exists::AttributeNotExists;
pub use attribute_type::AttributeType;
pub use begins_with::BeginsWith;
pub use between::Between;
pub use comparison::{
    equal, greater_than, greater_than_or_equal, less_than, less_than_or_equal, not_equal,
    Comparator, Comparison,
};
pub use contains::Contains;
pub use in_::In;
pub use not::Not;
pub use or::Or;
pub use parenthetical::Parenthetical;

use core::{fmt, ops};

/// Represents a logical condition in a [DynamoDB expression][1].
///
/// You will usually create these using the methods on [`Path`].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Syntax
/// [`Path`]: crate::path::Path
#[must_use = "Use in a DynamoDB expression with \
    `Expression::builder().with_condition(condition)` or \
    `Expression::builder().with_filter(condition)`"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    AttributeExists(AttributeExists),
    AttributeNotExists(AttributeNotExists),
    AttributeType(AttributeType),
    BeginsWith(BeginsWith),
    Between(Between),
    Contains(Contains),
    In(In),
    Not(Not),
    And(And),
    Or(Or),
    Comparison(Comparison),
    Parenthetical(Parenthetical),
}

impl Condition {
    /// A [DynamoDB logical `AND`][1] condition.
    ///
    /// See also: [`And`]
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
    pub fn and<R>(self, right: R) -> Self
    where
        R: Into<Condition>,
    {
        Self::And(And {
            left: self.into(),
            right: right.into().into(),
        })
    }

    /// A [DynamoDB logical `OR`][1] condition.
    ///
    /// See also: [`Or`]
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
    /// let condition = a.greater_than(b).or(c.less_than(d));
    /// assert_eq!("a > b OR c < d", condition.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.LogicalEvaluations
    pub fn or<R>(self, right: R) -> Self
    where
        R: Into<Condition>,
    {
        Self::Or(Or {
            left: self.into(),
            right: right.into().into(),
        })
    }

    /// A [DynamoDB logical `NOT`][1] condition.
    ///
    /// See also: [`Not`]
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use dynamodb_expression::Path;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let a = "a".parse::<Path>()?;
    /// let b = "b".parse::<Path>()?;
    ///
    /// let condition = a.greater_than(b).not();
    /// assert_eq!("NOT a > b", condition.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.LogicalEvaluations
    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Self {
        Self::Not(Not::from(self))
    }

    /// Wraps a condition in [parentheses][1].
    ///
    /// See also: [`Parenthetical`]
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
    /// let condition = a.greater_than(b).parenthesize().and(c.less_than(d).parenthesize());
    /// assert_eq!("(a > b) AND (c < d)", condition.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Parentheses
    pub fn parenthesize(self) -> Self {
        Self::Parenthetical(Parenthetical::from(self))
    }
}

impl ops::Not for Condition {
    type Output = Condition;

    /// A [DynamoDB logical `NOT`][1] condition.
    ///
    /// See also: [`Condition::not`], [`Not`]
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use dynamodb_expression::Path;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let a = "a".parse::<Path>()?;
    /// let b = "b".parse::<Path>()?;
    ///
    /// let condition = !a.greater_than(b);
    /// assert_eq!("NOT a > b", condition.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.LogicalEvaluations
    fn not(self) -> Self::Output {
        Condition::not(self)
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Condition::AttributeExists(condition) => condition.fmt(f),
            Condition::AttributeNotExists(condition) => condition.fmt(f),
            Condition::AttributeType(condition) => condition.fmt(f),
            Condition::BeginsWith(condition) => condition.fmt(f),
            Condition::Between(condition) => condition.fmt(f),
            Condition::Contains(condition) => condition.fmt(f),
            Condition::In(condition) => condition.fmt(f),
            Condition::Not(condition) => condition.fmt(f),
            Condition::And(condition) => condition.fmt(f),
            Condition::Or(condition) => condition.fmt(f),
            Condition::Comparison(condition) => condition.fmt(f),
            Condition::Parenthetical(condition) => condition.fmt(f),
        }
    }
}

impl From<AttributeExists> for Condition {
    fn from(condition: AttributeExists) -> Self {
        Self::AttributeExists(condition)
    }
}

impl From<AttributeNotExists> for Condition {
    fn from(condition: AttributeNotExists) -> Self {
        Self::AttributeNotExists(condition)
    }
}

impl From<AttributeType> for Condition {
    fn from(condition: AttributeType) -> Self {
        Self::AttributeType(condition)
    }
}

impl From<BeginsWith> for Condition {
    fn from(condition: BeginsWith) -> Self {
        Self::BeginsWith(condition)
    }
}

impl From<Between> for Condition {
    fn from(condition: Between) -> Self {
        Self::Between(condition)
    }
}

impl From<Contains> for Condition {
    fn from(condition: Contains) -> Self {
        Self::Contains(condition)
    }
}

impl From<In> for Condition {
    fn from(condition: In) -> Self {
        Self::In(condition)
    }
}

impl From<Not> for Condition {
    fn from(condition: Not) -> Self {
        Self::Not(condition)
    }
}

impl From<And> for Condition {
    fn from(condition: And) -> Self {
        Self::And(condition)
    }
}

impl From<Or> for Condition {
    fn from(condition: Or) -> Self {
        Self::Or(condition)
    }
}

impl From<Comparison> for Condition {
    fn from(condition: Comparison) -> Self {
        Self::Comparison(condition)
    }
}

impl From<Parenthetical> for Condition {
    fn from(condition: Parenthetical) -> Self {
        Self::Parenthetical(condition)
    }
}

// As of v0.29, `aws_sdk_dynamodb` wants an `Into<String>` to be passed to the
// `.filter_expression()` methods on its `*Input` types. So, we'll implement
// that to make this nicer to work with.
impl From<Condition> for String {
    fn from(condition: Condition) -> Self {
        // TODO: Is there a more efficient way when all of these require formatting?
        condition.to_string()
    }
}

#[cfg(test)]
pub(crate) mod test {
    use pretty_assertions::assert_eq;

    use crate::path::Path;

    use super::{
        comparison::{greater_than, less_than},
        Condition,
    };

    /// `a > b`
    pub fn cmp_a_gt_b() -> Condition {
        Condition::Comparison(greater_than(
            "a".parse::<Path>().unwrap(),
            "b".parse::<Path>().unwrap(),
        ))
    }

    /// `c < d`
    pub fn cmp_c_lt_d() -> Condition {
        Condition::Comparison(less_than(
            "c".parse::<Path>().unwrap(),
            "d".parse::<Path>().unwrap(),
        ))
    }

    #[test]
    fn display() {
        assert_eq!("a > b", cmp_a_gt_b().to_string());
        assert_eq!("c < d", cmp_c_lt_d().to_string());
    }

    #[test]
    fn and() {
        use crate::Path;
        use pretty_assertions::assert_eq;

        let a = "a".parse::<Path>().unwrap();
        let b = "b".parse::<Path>().unwrap();
        let c = "c".parse::<Path>().unwrap();
        let d = "d".parse::<Path>().unwrap();

        let condition = a.greater_than(b).and(c.less_than(d));
        assert_eq!("a > b AND c < d", condition.to_string());
    }

    #[test]
    fn or() {
        use crate::Path;
        use pretty_assertions::assert_eq;

        let a = "a".parse::<Path>().unwrap();
        let b = "b".parse::<Path>().unwrap();
        let c = "c".parse::<Path>().unwrap();
        let d = "d".parse::<Path>().unwrap();

        let condition = a.greater_than(b).or(c.less_than(d));
        assert_eq!("a > b OR c < d", condition.to_string());
    }

    #[test]
    fn not() {
        use crate::Path;
        use pretty_assertions::assert_eq;

        let a = "a".parse::<Path>().unwrap();
        let b = "b".parse::<Path>().unwrap();

        let condition = a.greater_than(b).not();
        assert_eq!("NOT a > b", condition.to_string());
    }

    #[test]
    fn not_operator() {
        use crate::Path;
        use pretty_assertions::assert_eq;

        let a = "a".parse::<Path>().unwrap();
        let b = "b".parse::<Path>().unwrap();

        let condition = !a.greater_than(b);
        assert_eq!("NOT a > b", condition.to_string());
    }
}
