use core::fmt;

use crate::{
    path::Path,
    value::{Value, ValueOrRef},
};

/// The [DynamoDB `contains` function][1]. True if the attribute specified
/// by [`Path`] is one of the following:
/// * A `String` that contains a particular substring.
/// * A `Set` that contains a particular element within the set.
/// * A `List` that contains a particular element within the list.
///
/// The operand must be a `String` if the attribute specified by path is a
/// `String`. If the attribute specified by path is a `Set`, the operand
/// must be the sets element type.
///
/// ```
/// use dynamodb_expression::{condition::Contains, Num, Path};
///
/// // String
/// let condition = Contains::new(Path::new_name("foo"), "Quinn");
/// assert_eq!(r#"contains(foo, "Quinn")"#, condition.to_string());
///
/// // Number
/// let condition = Contains::new(Path::new_name("foo"), Num::new(42));
/// assert_eq!(r#"contains(foo, 42)"#, condition.to_string());
///
/// // Binary
/// let condition = Contains::new(Path::new_name("foo"), Vec::<u8>::from("fish"));
/// assert_eq!(r#"contains(foo, "ZmlzaA==")"#, condition.to_string());
/// ```
///
/// See also: [`Path::contains`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contains {
    // `Path` is correct here
    // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Syntax
    pub(crate) path: Path,
    pub(crate) operand: ValueOrRef,
}

impl Contains {
    pub fn new<P, S>(path: P, operand: S) -> Self
    where
        P: Into<Path>,
        S: Into<Value>,
    {
        Self {
            path: path.into(),
            operand: operand.into().into(),
        }
    }
}

impl fmt::Display for Contains {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "contains({}, {})", self.path, self.operand)
    }
}
