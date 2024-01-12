use core::fmt::{self, Write};

use super::Condition;

/// Wraps a condition in parentheses.
///
/// See also: [`Condition::parenthesize`]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parenthetical {
    pub(crate) condition: Box<Condition>,
}

impl Parenthetical {
    // /// Unwrap nested parentheses. E.g., `(((a and (((b < c))))))` becomes `(a and (b < c))`
    // pub fn normalize(self) -> Condition {
    //     Self(
    //         self.flatten()
    //             .0
    //             // Normalize down the chain.
    //             .normalize()
    //             .into(),
    //     )
    //     .into()
    // }

    // /// Removes this level of nested parentheses without any deeper flattening or normalization.
    // /// E.g., `(((a and (((b < c))))))` becomes `(a and (((b < c))))`
    // pub fn flatten(self) -> Self {
    //     let mut inner = self.0;
    //     while let Expression::Parenthetical(Self(paren_inner)) = *inner {
    //         inner = paren_inner;
    //     }

    //     Self(inner)
    // }
}

impl<T> From<T> for Parenthetical
where
    T: Into<Box<Condition>>,
{
    fn from(condition: T) -> Self {
        Self {
            condition: condition.into(),
        }
    }
}

impl fmt::Display for Parenthetical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('(')?;
        self.condition.fmt(f)?;
        f.write_char(')')
    }
}

#[cfg(test)]
mod test {
    use std::io::{self, Write};

    use pretty_assertions::assert_str_eq;

    use crate::condition::test::cmp_a_gt_b;

    #[test]
    fn parentheses() {
        let expr = cmp_a_gt_b();

        for i in 0..3 {
            let mut wrapped = expr.clone().parenthesize();
            for _ in 0..i {
                wrapped = wrapped.parenthesize();
            }

            println!("{i}: {wrapped}");
            io::stdout().lock().flush().unwrap();

            assert_str_eq!(
                match i {
                    0 => format!("({expr})"),
                    1 => format!("(({expr}))"),
                    2 => format!("((({expr})))"),
                    _ => unreachable!(),
                },
                wrapped.to_string(),
                "The `Display` output wasn't what was expected."
            );

            // let normalized = wrapped.normalize();
            // println!(" → {normalized}");
            // assert_str_eq!(
            //     "(a > b)",
            //     normalized.to_string(),
            //     "Should always normalize to a single set of parentheses."
            // );
        }
    }
}
