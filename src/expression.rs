use core::{
    fmt::{self, Display},
    ops,
};

use crate::{function::Function, not::Not, parenthetical::Parenthetical, Comparator};

/**
The type to build a DynamoDB filter or condition expression. The `Display` output
of an instance on this type will provide the needed expression.

[DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)

```no-compile
condition-expression ::=
      operand comparator operand
    | operand BETWEEN operand AND operand
    | operand IN ( operand (',' operand (, ...) ))
    | function
    | condition AND condition
    | condition OR condition
    | NOT condition
    | ( condition )
```
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Comparison(String, Comparator, String),
    Between(String, String, String),
    In(String, Vec<String>),
    Function(Function),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Not),
    Parenthetical(Parenthetical),
}

impl Expression {
    pub fn and(self, b: Expression) -> Self {
        Self::And(self.into(), b.into())
    }

    pub fn or(self, b: Expression) -> Self {
        Self::Or(self.into(), b.into())
    }

    pub fn parenthesize(self) -> Self {
        Self::Parenthetical(self.into())
    }

    pub fn normalize(self) -> Self {
        use Expression::*;

        match self {
            Comparison(_, _, _) => self,
            Between(_, _, _) => self,
            In(_, _) => self,
            Function(_) => self,
            And(a, b) => And(a.normalize().into(), b.normalize().into()),
            Or(a, b) => Or(a.normalize().into(), b.normalize().into()),
            Not(a) => a.normalize(),
            Parenthetical(a) => a.normalize().into(),
        }
    }
}

impl ops::Not for Expression {
    type Output = Self;

    fn not(self) -> Self::Output {
        Not::from(self).into()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Comparison(a, cmp, b) => write!(f, "{a} {cmp} {b}"),
            Self::Between(a, b, c) => write!(f, "{a} BETWEEN {b} AND {c}"),
            Self::In(a, b) => write!(f, "{a} IN ({})", b.join(",")),
            Self::Function(function) => f.write_str(&function.to_string()),
            Self::And(a, b) => write!(f, "{a} AND {b}"),
            Self::Or(a, b) => write!(f, "{a} OR {b}"),
            Self::Not(a) => f.write_str(&a.to_string()),
            Self::Parenthetical(a) => f.write_str(&a.to_string()),
        }
    }
}

impl From<Function> for Expression {
    fn from(function: Function) -> Self {
        Self::Function(function)
    }
}

impl From<Not> for Expression {
    fn from(not: Not) -> Self {
        Self::Not(not)
    }
}

impl From<Parenthetical> for Expression {
    fn from(parenthetical: Parenthetical) -> Self {
        Self::Parenthetical(parenthetical)
    }
}

// As of v0.29, `aws_sdk_dynamodb` wants an `Into<String>` to be passed to the
// `.filter_expression()` methods on its `*Input` types. So, we'll implement
// that to make it nicer to work with.
impl From<Expression> for String {
    fn from(value: Expression) -> Self {
        value.to_string()
    }
}

impl From<&Expression> for String {
    fn from(value: &Expression) -> Self {
        value.to_string()
    }
}

/// Compare two values.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
pub fn cmp<A, B>(a: A, cmp: Comparator, b: B) -> Expression
where
    A: Into<String>,
    B: Into<String>,
{
    Expression::Comparison(a.into(), cmp, b.into())
}

/// Check if a value is between two others.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
pub fn between<A, B, C>(a: A, b: B, c: C) -> Expression
where
    A: Into<String>,
    B: Into<String>,
    C: Into<String>,
{
    Expression::Between(a.into(), b.into(), c.into())
}

/// `a IN (b[, ..])` — true if a is equal to any value in the list.
///
/// The list can contain up to 100 values. It must have at least 1.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
pub fn in_<A, B, I>(a: A, b: I) -> Result<Expression, Vec<String>>
where
    A: Into<String>,
    B: Into<String>,
    I: IntoIterator<Item = B>,
{
    let b: Vec<_> = b.into_iter().map(Into::into).collect();
    if b.is_empty() || b.len() > 100 {
        return Err(b);
    }

    Ok(Expression::In(a.into(), b))
}

/// Note that the [`function`] module has helper functions to produce [`Expression`]s.
/// These are also re-exported from the root of this crate.
///
/// [`function`]: crate::function
pub fn function(function: Function) -> Expression {
    function.into()
}

/// Negates the expression with `NOT`.
/// E.g., `a < b` becomes `NOT a < b`.
///
/// Tip: you can use `!a` or `a.not()` to do this, instead.
pub fn not(a: Expression) -> Expression {
    !a
}

/// Wraps an expression in parentheses.
/// E.g., `a < b AND c > d` becomes `(a < b AND c > d)`.
///
/// Tip: you can use `a.parenthesize()` to do this, instead.
pub fn parenthesize(a: Expression) -> Expression {
    a.parenthesize()
}

#[cfg(test)]
pub(crate) mod test {
    use pretty_assertions::{assert_eq, assert_str_eq};

    use super::*;
    use crate::comparator::Comparator::*;

    #[test]
    fn display_comparison() {
        assert_str_eq!("a > b", cmp("a", GT, "b").to_string());
        assert_str_eq!("c < d", cmp("c", LT, "d").to_string());
    }

    #[test]
    fn display_between() {
        assert_str_eq!("a BETWEEN b AND c", between("a", "b", "c").to_string());
        assert_str_eq!("d BETWEEN e AND f", between("d", "e", "f").to_string());
    }

    #[test]
    fn display_in() {
        assert_str_eq!(
            "a IN (b,c,d)",
            in_("a", ["b", "c", "d"]).unwrap().to_string()
        );
        assert_str_eq!("e IN (f)", in_("e", ["f"]).unwrap().to_string());
    }

    #[test]
    fn display_or() {
        assert_str_eq!(
            "a >= b OR c <> d",
            cmp("a", GE, "b").or(cmp("c", NE, "d")).to_string()
        );
        assert_str_eq!(
            "a BETWEEN b AND c OR d IN (e,f)",
            between("a", "b", "c")
                .or(in_("d", ["e", "f"]).unwrap())
                .to_string()
        );
    }

    #[test]
    fn display_and() {
        assert_str_eq!(
            "a >= b AND c <> d",
            cmp("a", GE, "b").and(cmp("c", NE, "d")).to_string()
        );
        assert_str_eq!(
            "a BETWEEN b AND c AND d IN (e,f)",
            between("a", "b", "c")
                .and(in_("d", ["e", "f"]).unwrap())
                .to_string()
        );
    }

    #[test]
    fn normalize_and() {
        let wrapped = paren3(paren3(cmp_a_gt_b()).and(paren3(cmp_c_lt_d())));
        let normalized = wrapped.clone().normalize();

        println!("{wrapped}");
        println!("{normalized}");

        assert_eq!(
            paren(paren(cmp_a_gt_b()).and(paren(cmp_c_lt_d()))),
            normalized
        );
    }

    #[test]
    fn normalize_or() {
        let wrapped = paren3(paren3(cmp_a_gt_b()).or(paren3(cmp_c_lt_d())));
        let normalized = wrapped.clone().normalize();

        println!("{wrapped}");
        println!("{normalized}");

        assert_eq!(
            paren(paren(cmp_a_gt_b()).or(paren(cmp_c_lt_d()))),
            normalized
        );
    }

    pub fn paren(cond: Expression) -> Expression {
        parenthesize(cond)
    }

    pub fn paren2(cond: Expression) -> Expression {
        paren(paren(cond))
    }

    pub fn paren3(cond: Expression) -> Expression {
        paren2(paren(cond))
    }

    /// `a > b`
    pub fn cmp_a_gt_b() -> Expression {
        cmp("a", GT, "b")
    }

    /// `c < d`
    pub fn cmp_c_lt_d() -> Expression {
        cmp("c", LT, "d")
    }
}
