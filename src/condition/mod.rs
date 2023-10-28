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

/// Represents a [DynamoDB condition or filter expression][1].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Syntax
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    /// A [logical `AND`][1] operation
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

    /// A [logical `OR`][1] operation
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

    /// A [logical `NOT`][1] operation
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.LogicalEvaluations
    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Self {
        Self::Not(Not::from(self))
    }

    /// Wraps a condition in [parentheses][1].
    /// For example, `a < b AND c > d` becomes `(a < b AND c > d)`.
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Parentheses
    pub fn parenthesize(self) -> Self {
        Self::Parenthetical(self.into())
    }
}

impl ops::Not for Condition {
    type Output = Condition;

    /// A [logical `NOT`][1] operation
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
// that to make it nicer to work with.
impl From<Condition> for String {
    fn from(condition: Condition) -> Self {
        condition.to_string()
    }
}

#[cfg(test)]
pub(crate) mod test {
    use pretty_assertions::assert_str_eq;

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
        assert_str_eq!("a > b", cmp_a_gt_b().to_string());
        assert_str_eq!("c < d", cmp_c_lt_d().to_string());
    }
}
