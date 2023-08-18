use core::fmt::{self, Display};

/**
[DynamoDB comparison operators](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)

```no-compile
comparator ::=
    =
    | <>
    | <
    | <=
    | >
    | >=
*/
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Comparator {
    /// Equal (`=`)
    EQ,
    /// Not equal (`<>`)
    NE,
    /// Less than (`<`)
    LT,
    /// Less than or equal (`<=`)
    LE,
    /// Greater than (`>`)
    GT,
    /// Greater than or equal (`>=`)
    GE,
}

impl Display for Comparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Comparator::EQ => "=",
            Comparator::NE => "<>",
            Comparator::LT => "<",
            Comparator::LE => "<=",
            Comparator::GT => ">",
            Comparator::GE => ">=",
        })
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_str_eq;

    use super::Comparator::*;

    #[test]
    fn display() {
        assert_str_eq!("=", EQ.to_string());
        assert_str_eq!("<>", NE.to_string());
        assert_str_eq!("<", LT.to_string());
        assert_str_eq!("<=", LE.to_string());
        assert_str_eq!(">", GT.to_string());
        assert_str_eq!(">=", GE.to_string());
    }
}
