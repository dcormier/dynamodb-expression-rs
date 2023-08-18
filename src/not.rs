use core::fmt;

use crate::{expression::Expression, parenthetical::Parenthetical};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Not(pub Box<Expression>);

impl Not {
    /// Normalizes pairs of `NOT` statements by removing them. E.g.,
    /// `NOT NOT a < b` becomes `a < b`.
    /// `NOT (NOT a < b)` becomes `a < b`.
    pub fn normalize(self) -> Expression {
        // `NOT inner`

        if let Expression::Not(Self(inner)) = *self.0 {
            // `NOT NOT inner`
            inner.normalize()
        } else if let Expression::Parenthetical(parens) = *self.0 {
            // Flatten nested paren statements to turn `NOT (((inner)))` into `NOT (inner)`
            let Parenthetical(mut inner) = parens.flatten();

            // `NOT (inner)`

            if let Expression::Not(Self(inner)) = *inner {
                // `NOT (NOT inner)`
                inner.normalize()
            } else {
                // `NOT (inner)
                //
                // Put it back in the parentheses.
                inner = Expression::from(Parenthetical::from(inner.normalize())).into();

                // Put it back in `NOT`
                Self::from(inner).into()
            }
        } else {
            self.into()
        }
    }
}

impl<T> From<T> for Not
where
    T: Into<Box<Expression>>,
{
    fn from(expression: T) -> Self {
        Self(expression.into())
    }
}

impl fmt::Display for Not {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NOT {}", self.0)
    }
}

#[cfg(test)]
mod test {
    use std::io::{self, Write};

    use pretty_assertions::assert_str_eq;

    use super::Not;
    use crate::expression::{
        test::{cmp_a_gt_b, paren3},
        Expression,
    };

    #[test]
    fn display() {
        assert_str_eq!("NOT a > b", (!cmp_a_gt_b()).to_string());
    }

    #[test]
    fn not() {
        let expr = cmp_a_gt_b();

        for i in 0..3 {
            let mut wrapped = Not::from(expr.clone());
            for _ in 0..i {
                wrapped = Not::from(Expression::from(wrapped));
            }

            print!("{i}: {wrapped}");
            io::stdout().lock().flush().unwrap();

            assert_str_eq!(
                match i {
                    0 => format!("NOT {expr}"),
                    1 => format!("NOT NOT {expr}"),
                    2 => format!("NOT NOT NOT {expr}"),
                    _ => unreachable!(),
                },
                wrapped.to_string(),
            );

            let normalized = wrapped.normalize();
            println!(" → {normalized}");
            assert_str_eq!(
                if i % 2 == 1 { "a > b" } else { "NOT a > b" },
                normalized.to_string(),
                "Pairs of `NOT`s cancel each other out."
            );
        }
    }

    #[test]
    fn not_parens() {
        let expr = cmp_a_gt_b();

        for i in 0..3 {
            let mut wrapped = Not::from(expr.clone());
            for _ in 0..i {
                wrapped = Not::from(Expression::from(wrapped).parenthesize().parenthesize());
            }

            print!("{i}: {wrapped}");
            io::stdout().lock().flush().unwrap();

            let (expected_wrapped, expected_normalized) = match i {
                0 => {
                    let expr = format!("NOT {expr}");
                    (expr.clone(), expr)
                }
                1 => (format!("NOT ((NOT {expr}))"), expr.to_string()),
                2 => (
                    format!("NOT ((NOT ((NOT {expr}))))"),
                    format!("(NOT {expr})"),
                ),
                _ => unreachable!(),
            };

            assert_str_eq!(expected_wrapped, wrapped.to_string());

            let normalized = wrapped.normalize();
            println!(" → {normalized}");
            assert_str_eq!(
                expected_normalized,
                normalized.to_string(),
                "Pairs of `NOT`s cancel each other out."
            );
        }
    }

    #[test]
    fn normalize_variants() {
        let wrapped = paren3(!paren3(cmp_a_gt_b()));
        let normalized = wrapped.clone().normalize();

        println!("{wrapped}");
        println!("{normalized}");

        assert_str_eq!(
            (!cmp_a_gt_b().parenthesize()).parenthesize().to_string(),
            normalized.to_string()
        );

        // ----

        let wrapped = !(!paren3(cmp_a_gt_b()));
        let normalized = wrapped.clone().normalize();

        println!("{wrapped}");
        println!("{normalized}");

        assert_str_eq!(
            cmp_a_gt_b().parenthesize().to_string(),
            normalized.to_string(),
            "`NOT NOT` should be normalized away"
        );

        // ----

        let wrapped = !((!paren3(cmp_a_gt_b())).parenthesize());
        let normalized = wrapped.clone().normalize();

        println!("{wrapped}");
        println!("{normalized}");

        assert_str_eq!(
            cmp_a_gt_b().parenthesize().to_string(),
            normalized.to_string(),
            "`NOT (NOT` should be normalized away"
        );

        // ----

        let wrapped = !!!paren3(cmp_a_gt_b());
        let normalized = wrapped.clone().normalize();

        println!("{wrapped}");
        println!("{normalized}");

        assert_str_eq!(
            (!cmp_a_gt_b().parenthesize()).to_string(),
            normalized.to_string(),
            "`NOT NOT NOT` should be normalized to `NOT`"
        );
    }
}
