pub mod append;

use core::fmt::{self, Write};

use crate::{
    path::Path,
    value::{scalar::Num, Value},
};

use self::append::Append;

// func Set(name NameBuilder, operandBuilder OperandBuilder) UpdateBuilder
// func (ub UpdateBuilder) Set(name NameBuilder, operandBuilder OperandBuilder) UpdateBuilder

/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Set {
    actions: Vec<SetAction>,
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        self.actions.iter().try_for_each(|action| {
            if first {
                first = false
            } else {
                f.write_str(", ")?;
            }

            action.fmt(f)
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SetAction {
    // TODO: This needs to support paths like:
    // "SET RelatedItems = :ri, ProductReviews = :pr"
    // "SET RelatedItems[1] = :ri"
    // "SET #pr.#5star[1] = :r5, #pr.#3star = :r3"
    // See:
    //      https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.AddingNestedMapAttributes
    //      https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html
    Assign(Assign),

    /// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.IncrementAndDecrement>
    Math(Math),

    /// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.UpdatingListElements>
    Append(Append),

    /// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.PreventingAttributeOverwrites>
    IfNotExists(IfNotExists),
}

impl From<Assign> for SetAction {
    fn from(assign: Assign) -> Self {
        Self::Assign(assign)
    }
}

impl From<Math> for SetAction {
    fn from(math: Math) -> Self {
        Self::Math(math)
    }
}

impl From<Append> for SetAction {
    fn from(append: Append) -> Self {
        Self::Append(append)
    }
}

impl From<IfNotExists> for SetAction {
    fn from(if_not_exists: IfNotExists) -> Self {
        Self::IfNotExists(if_not_exists)
    }
}

impl fmt::Display for SetAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetAction::Assign(action) => action.fmt(f),
            SetAction::Math(action) => action.fmt(f),
            SetAction::Append(action) => action.fmt(f),
            SetAction::IfNotExists(action) => action.fmt(f),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum MathOp {
    Add,
    Sub,
}

impl fmt::Debug for MathOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for MathOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            MathOp::Add => '+',
            MathOp::Sub => '-',
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Assign {
    /// `Path` is correct, here.
    /// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.AddingListElements>
    /// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.AddingNestedMapAttributes>
    pub(crate) path: Path,
    pub(crate) value: Value,
}

impl fmt::Display for Assign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.path, self.value)
    }
}

/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.IncrementAndDecrement>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Math {
    // TODO: Name or Path?
    dst: Path,
    src: Path,
    op: MathOp,
    value: Num,
}

impl fmt::Display for Math {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            dst,
            src,
            op,
            value,
        } = self;

        write!(f, "{dst} = {src} {op} {value}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfNotExists {
    dst: Path,
    src: Path,
    value: Value,
}

impl IfNotExists {
    /// For setting a field if it does not exist
    pub fn new_for_self<P, V>(path: P, value: V) -> Self
    where
        P: Clone + Into<Path>,
        V: Into<Value>,
    {
        Self::new_with_source(path.clone(), path, value)
    }

    /// For setting a field if a (potentially different) field does not exist.
    pub fn new_with_source<D, S, V>(destination: D, source: S, value: V) -> Self
    where
        D: Into<Path>,
        S: Into<Path>,
        V: Into<Value>,
    {
        fn new_with_source(dst: Path, src: Path, value: Value) -> IfNotExists {
            IfNotExists { dst, src, value }
        }

        new_with_source(destination.into(), source.into(), value.into())
    }
}

impl fmt::Display for IfNotExists {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { dst, src, value } = self;

        write!(f, "{dst} = if_not_exists({src}, {value})")
    }
}
