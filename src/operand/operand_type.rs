use core::fmt;

use crate::{
    condition::Condition,
    operand::Size,
    path::{Element, IndexedField, Name, Path},
    value::{Ref, Scalar, ValueOrRef},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OperandType {
    Path(Path),
    Scalar(ValueOrRef),
    Condition(Box<Condition>),
    Size(Size),
}

impl fmt::Display for OperandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Path(operand) => operand.fmt(f),
            Self::Scalar(operand) => operand.fmt(f),
            Self::Condition(operand) => operand.fmt(f),
            Self::Size(operand) => operand.fmt(f),
        }
    }
}

impl From<Path> for OperandType {
    fn from(path: Path) -> Self {
        Self::Path(path)
    }
}

impl From<Element> for OperandType {
    fn from(element: Element) -> Self {
        Self::Path(element.into())
    }
}

impl From<Name> for OperandType {
    fn from(name: Name) -> Self {
        Self::Path(name.into())
    }
}

impl From<IndexedField> for OperandType {
    fn from(field: IndexedField) -> Self {
        Self::Path(field.into())
    }
}

impl From<Scalar> for OperandType {
    fn from(value: Scalar) -> Self {
        Self::Scalar(value.into())
    }
}

impl From<Ref> for OperandType {
    fn from(value: Ref) -> Self {
        Self::Scalar(value.into())
    }
}

impl From<Condition> for OperandType {
    fn from(condition: Condition) -> Self {
        Self::Condition(condition.into())
    }
}

impl From<Box<Condition>> for OperandType {
    fn from(condition: Box<Condition>) -> Self {
        Self::Condition(condition)
    }
}

impl From<Size> for OperandType {
    fn from(size: Size) -> Self {
        Self::Size(size)
    }
}
