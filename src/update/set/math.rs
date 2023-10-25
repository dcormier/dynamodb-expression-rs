use core::fmt::{self, Write};

use crate::{
    path::Path,
    value::{scalar::Num, ValueOrRef},
};

/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.IncrementAndDecrement>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Math {
    // TODO: Name or Path for these?
    pub(crate) dst: Path,
    pub(crate) src: Option<Path>,
    op: MathOp,
    pub(crate) num: ValueOrRef,
}

/// A [math operation][1] to modify a field and assign the updated value
/// to another (possibly different) field.
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.IncrementAndDecrement
impl Math {
    pub fn builder<T>(dst: T) -> Builder
    where
        T: Into<Path>,
    {
        Builder {
            dst: dst.into(),
            src: None,
        }
    }
}

impl fmt::Display for Math {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { dst, src, op, num } = self;

        // If no source field is specified, default to using the destination field.
        let src = src.as_ref().unwrap_or(dst);

        write!(f, "{dst} = {src} {op} {num}")
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

#[must_use = "Consume the `Builder` using its `.add()` or `.sub()` methods"]
#[derive(Debug, Clone)]
pub struct Builder {
    dst: Path,
    src: Option<Path>,
}

/// Builds a [`Math`] instance. Create an instance of this by using [`Math::builder`].
impl Builder {
    /// Sets the source field to read the initial value from.
    /// Defaults to the destination field.
    pub fn src<T>(mut self, src: T) -> Self
    where
        T: Into<Path>,
    {
        self.src = Some(src.into());

        self
    }

    /// Sets addition as the operation to perform.
    pub fn add(self) -> BuilderOp {
        self.with_op(MathOp::Add)
    }

    /// Sets subtraction as the operation to perform.
    pub fn sub(self) -> BuilderOp {
        self.with_op(MathOp::Sub)
    }

    fn with_op(self, op: MathOp) -> BuilderOp {
        BuilderOp { builder: self, op }
    }
}

/// Builds a [`Math`] instance.
/// Consume this by using the [`BuilderOp::num`] method.
///
/// Create an instance of this by starting with [`Math::builder`].
#[must_use = "Consume the `BuilderOp` using its `.num()` method"]
#[derive(Debug, Clone)]
pub struct BuilderOp {
    builder: Builder,
    op: MathOp,
}

impl BuilderOp {
    /// Sets the number to add/subtract against the specified field.
    ///
    /// Builds the [`Math`] instance.
    pub fn num<T>(self, num: T) -> Math
    where
        T: Into<Num>,
    {
        let Self { builder, op } = self;
        let Builder { dst, src } = builder;

        Math {
            dst,
            src,
            op,
            num: num.into().into(),
        }
    }
}
