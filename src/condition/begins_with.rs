use core::fmt;

use crate::{
    name::Name,
    value::{Scalar, ValueOrRef},
};

/// True if the attribute specified by `path` begins with a particular substring.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BeginsWith {
    pub(crate) path: Name,
    pub(crate) substr: ValueOrRef,
}

impl BeginsWith {
    pub fn new<P, S>(path: P, substr: S) -> Self
    where
        P: Into<Name>,
        S: Into<String>,
    {
        Self {
            path: path.into(),
            substr: Scalar::from(substr.into()).into(),
        }
    }
}

impl fmt::Display for BeginsWith {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "begins_with({}, {})", self.path, self.substr)
    }
}
