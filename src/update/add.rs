use core::fmt;

use crate::{
    name::Name,
    value::{scalar::Num, Set, Value},
};

// func Add(name NameBuilder, value ValueBuilder) UpdateBuilder
// func (ub UpdateBuilder) Add(name NameBuilder, value ValueBuilder) UpdateBuilder

/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.ADD>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Add {
    // TODO: Name or Path?
    pub(crate) name: Name,
    pub(crate) value: Value,
}

impl Add {
    pub fn new<N, V>(name: N, value: V) -> Self
    where
        N: Into<Name>,
        V: Into<AddValue>,
    {
        Self {
            name: name.into(),
            value: match value.into() {
                AddValue::Num(num) => Value::Scalar(num.into()),
                AddValue::Set(set) => set.into(),
            },
        }
    }
}

impl fmt::Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD {} {}", self.name, self.value)
    }
}

/// A value that can be used for the `ADD` operation in a DynamoDB update request.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AddValue {
    Set(Set),
    Num(Num),
}

impl fmt::Display for AddValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Set(value) => value.fmt(f),
            Self::Num(value) => value.fmt(f),
        }
    }
}

impl From<Set> for AddValue {
    fn from(value: Set) -> Self {
        Self::Set(value)
    }
}

impl From<Num> for AddValue {
    fn from(value: Num) -> Self {
        Self::Num(value)
    }
}
