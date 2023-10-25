pub mod append;
pub mod math;

use core::fmt;

use crate::{
    path::Path,
    value::{Value, ValueOrRef},
};

pub use self::append::Append;
pub use self::math::Math;

// func Set(name NameBuilder, operandBuilder OperandBuilder) UpdateBuilder
// func (ub UpdateBuilder) Set(name NameBuilder, operandBuilder OperandBuilder) UpdateBuilder

/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Set {
    pub(crate) actions: Vec<SetAction>,
}

impl Set {
    /// Add an additional action to this `SET` expression.
    pub fn and<T>(mut self, action: T) -> Self
    where
        T: Into<SetAction>,
    {
        self.actions.push(action.into());

        self
    }
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SET ")?;

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

impl<T> From<T> for Set
where
    T: Into<SetAction>,
{
    fn from(value: T) -> Self {
        Self {
            actions: vec![value.into()],
        }
    }
}

impl<T> FromIterator<T> for Set
where
    T: Into<SetAction>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            actions: iter.into_iter().map(Into::into).collect(),
        }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Assign {
    /// `Path` is correct, here.
    /// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.AddingListElements>
    /// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.AddingNestedMapAttributes>
    pub(crate) path: Path,
    pub(crate) value: ValueOrRef,
}

impl Assign {
    pub fn new<P, V>(path: P, value: V) -> Self
    where
        P: Into<Path>,
        V: Into<Value>,
    {
        Self {
            path: path.into(),
            value: value.into().into(),
        }
    }
}

impl fmt::Display for Assign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.path, self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfNotExists {
    pub(crate) dst: Path,
    pub(crate) src: Path,
    pub(crate) value: ValueOrRef,
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
        fn new_with_source(dst: Path, src: Path, value: ValueOrRef) -> IfNotExists {
            IfNotExists { dst, src, value }
        }

        new_with_source(destination.into(), source.into(), value.into().into())
    }
}

impl fmt::Display for IfNotExists {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { dst, src, value } = self;

        write!(f, "{dst} = if_not_exists({src}, {value})")
    }
}
