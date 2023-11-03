use core::fmt;

use crate::condition::Condition;

/// A [logical `NOT`][1] operation.
///
/// See: [`Condition`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.LogicalEvaluations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Not {
    pub(crate) condition: Box<Condition>,
}

impl Not {
    // /// Normalizes pairs of `NOT` statements by removing them. E.g.,
    // /// `NOT NOT a < b` becomes `a < b`.
    // /// `NOT (NOT a < b)` becomes `a < b`.
    // pub fn normalize(self) -> Expression {
    //     // `NOT inner`

    //     if let Expression::Logical(Logical::Not(Self(inner))) = *self.0 {
    //         // `NOT NOT inner`
    //         inner.normalize()
    //     } else if let Expression::Parenthetical(parens) = *self.0 {
    //         // `NOT (inner)`

    //         // Flatten nested paren statements to turn `NOT (((inner)))` into `NOT (inner)`
    //         let Parenthetical(inner) = parens.flatten();

    //         if let Expression::Logical(Logical::Not(Self(inner))) = *inner {
    //             // `NOT (NOT inner)`
    //             inner.normalize()
    //         } else {
    //             // `NOT (inner)
    //             //
    //             // Put it back in the parentheses.
    //             let inner = inner.normalize().parenthesize();

    //             // Put it back in `NOT`
    //             Self::from(inner).into()
    //         }
    //     } else {
    //         Expression::Logical(Logical::Not(self))
    //     }
    // }
}

impl<T> From<T> for Not
where
    T: Into<Box<Condition>>,
{
    fn from(condition: T) -> Self {
        Self {
            condition: condition.into(),
        }
    }
}

impl fmt::Display for Not {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NOT {}", self.condition)
    }
}

#[cfg(test)]
mod test {
    use std::io::{self, Write};

    use pretty_assertions::assert_str_eq;

    use crate::condition::{test::cmp_a_gt_b, Condition};

    use super::Not;

    #[test]
    fn display() {
        assert_str_eq!("NOT a > b", (!cmp_a_gt_b()).to_string());
    }

    #[test]
    fn not_expression() {
        let expr = cmp_a_gt_b();

        for i in 0..3 {
            let mut wrapped = !expr.clone();
            for _ in 0..i {
                wrapped = !wrapped;
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

            // let normalized = wrapped.normalize();
            // println!(" → {normalized}");
            // assert_str_eq!(
            //     if i % 2 == 1 { "a > b" } else { "NOT a > b" },
            //     normalized.to_string(),
            //     "Pairs of `NOT`s cancel each other out."
            // );
        }
    }

    #[test]
    fn not_parens() {
        let expr = cmp_a_gt_b();

        for i in 0..3 {
            let mut wrapped = Not::from(expr.clone());
            for _ in 0..i {
                wrapped = Not::from(Condition::Not(wrapped).parenthesize().parenthesize());
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

            _ = expected_normalized;
            // let normalized = wrapped.normalize();
            // println!(" → {normalized}");
            // assert_str_eq!(
            //     expected_normalized,
            //     normalized.to_string(),
            //     "Pairs of `NOT`s cancel each other out."
            // );
        }
    }

    #[test]
    fn normalize_variants() {
        let wrapped = cmp_a_gt_b()
            .parenthesize()
            .parenthesize()
            .parenthesize()
            .not()
            .parenthesize()
            .parenthesize()
            .parenthesize();

        println!("{wrapped}");

        assert_str_eq!("(((NOT (((a > b))))))", wrapped.to_string());

        // let normalized = wrapped.clone().normalize();
        // println!("{normalized}");

        // assert_str_eq!(
        //     cmp_a_gt_b().parenthesize().not().parenthesize().to_string(),
        //     normalized.to_string()
        // );

        // ----

        let wrapped = cmp_a_gt_b()
            .parenthesize()
            .parenthesize()
            .parenthesize()
            .not()
            .not();

        println!("{wrapped}");

        assert_str_eq!("NOT NOT (((a > b)))", wrapped.to_string());

        // let normalized = wrapped.clone().normalize();

        // println!("{normalized}");

        // assert_str_eq!(
        //     cmp_a_gt_b().parenthesize().to_string(),
        //     normalized.to_string(),
        //     "`NOT NOT` should be normalized away"
        // );

        // ----

        let wrapped = cmp_a_gt_b()
            .parenthesize()
            .parenthesize()
            .parenthesize()
            .not()
            .parenthesize()
            .not();

        println!("{wrapped}");

        assert_str_eq!("NOT (NOT (((a > b))))", wrapped.to_string());

        // let normalized = wrapped.clone().normalize();

        // println!("{normalized}");

        // assert_str_eq!(
        //     cmp_a_gt_b().parenthesize().to_string(),
        //     normalized.to_string(),
        //     "`NOT (NOT` should be normalized away"
        // );

        // ----

        let wrapped = !!!(cmp_a_gt_b().parenthesize().parenthesize().parenthesize());

        println!("{wrapped}");

        assert_str_eq!("NOT NOT NOT (((a > b)))", wrapped.to_string());

        // let normalized = wrapped.clone().normalize();

        // println!("{normalized}");

        // assert_str_eq!(
        //     (!cmp_a_gt_b().parenthesize()).to_string(),
        //     normalized.to_string(),
        //     "`NOT NOT NOT` should be normalized to `NOT`"
        // );
    }
}
