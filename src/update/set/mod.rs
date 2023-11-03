pub mod if_not_exists;
pub mod list_append;
pub mod math;

pub use self::if_not_exists::IfNotExists;
pub use self::list_append::ListAppend;
pub use self::math::Math;

use core::fmt;

use crate::{
    path::Path,
    value::{Value, ValueOrRef},
};

/// Represents a [`SET` statement for an update expression][1].
///
/// See also: [`Update`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET
/// [`Update`]: crate::update::Update
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Set {
    pub(crate) actions: Vec<SetAction>,
}

impl Set {
    /// Add an additional action to this `SET` statement.
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

/// Represents an action to take in a [`SET` statement][1] for an update expression.
///
/// See also: [`Set`], [`Update`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET
/// [`Update`]: crate::update::Update
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetAction {
    /// Assign a value in a `SET` statement for an update expression.
    ///
    /// See also: [`Assign`]
    Assign(Assign),

    /// Perform [math against a value in a `SET` statement][1] for an update expression.
    ///
    /// See also: [`Math`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.IncrementAndDecrement
    Math(Math),

    /// [Add values to a list][1] in a `SET` statement for an update expression.
    ///
    /// See also: [`ListAppend`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.UpdatingListElements
    ListAppend(ListAppend),

    /// Assign a value [only if it doesn't exist][1].
    ///
    /// See also: [`IfNotExists`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.PreventingAttributeOverwrites
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

impl From<ListAppend> for SetAction {
    fn from(append: ListAppend) -> Self {
        Self::ListAppend(append)
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
            SetAction::ListAppend(action) => action.fmt(f),
            SetAction::IfNotExists(action) => action.fmt(f),
        }
    }
}

/// Represents assigning a value of a [field][1], [list][2], or [map][3].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.ModifyingAttributes
/// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.AddingListElements
/// [3]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.AddingNestedMapAttributes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assign {
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
