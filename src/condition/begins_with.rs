use alloc::borrow::Cow;
use core::fmt::{self, Write};

use crate::{string_value, Name, ScalarValue};

/// True if the attribute specified by `path` begins with a particular substring.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BeginsWith {
    pub path: Name,
    pub substr: ScalarValue,
}

impl BeginsWith {
    pub fn new<P, S>(path: P, substr: S) -> Self
    where
        P: Into<Name>,
        S: Into<Cow<'static, str>>,
    {
        Self {
            path: path.into(),
            substr: string_value(substr.into()),
        }
    }
}

impl fmt::Display for BeginsWith {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("begins_with(")?;
        self.path.fmt(f)?;
        f.write_str(", ")?;
        self.substr.fmt(f)?;
        f.write_char(')')
    }
}
