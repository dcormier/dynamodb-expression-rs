use core::fmt;

use crate::{
    name::Name,
    value::{Scalar, ValueOrRef},
};

/// True if the attribute specified by `path` is one of the following:
/// * A `String` that contains a particular substring.
/// * A `Set` that contains a particular element within the set.
/// * A `List` that contains a particular element within the list.
///
/// The operand must be a `String` if the attribute specified by path is a `String`.
/// If the attribute specified by path is a `Set`, the operand must be the set's element type.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Contains {
    pub(crate) path: Name,
    pub(crate) operand: ValueOrRef,
}

impl Contains {
    pub fn new<P, S>(path: P, operand: S) -> Self
    where
        P: Into<Name>,
        S: Into<Scalar>,
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
