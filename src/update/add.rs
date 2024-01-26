use core::fmt::{self, Write};

use crate::{
    path::Path,
    value::{BinarySet, Num, NumSet, Ref, Set, StringSet, Value, ValueOrRef},
};

use super::Update;

/// Represents an [`ADD` statement][1] in a [DynamoDB update expression][2].
///
/// Prefer [`Path::add`] over this.
///
/// See also: [`Update`], [`Set`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.ADD
/// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html
#[must_use = "Use in an update expression with `Update::from(add)`"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Add {
    pub(crate) actions: Vec<AddAction>,
}

impl Add {
    /// Creates an [`Add`] for the specified [`Path`] and value.
    ///
    /// Prefer [`Path::add`] over this.
    pub fn new<N, V>(path: N, value: V) -> Self
    where
        N: Into<Path>,
        V: Into<AddValue>,
    {
        Self::from(AddAction::new(path, value))
    }

    /// Add an additional [`Update`] statement to this `ADD` statement.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let update = "foo"
    ///     .parse::<Path>()?
    ///     .add(Num::new(7))
    ///     .and("bar".parse::<Path>()?.set("a value"))
    ///     .and("baz".parse::<Path>()?.remove());
    /// assert_eq!(r#"SET bar = "a value" REMOVE baz ADD foo 7"#, update.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn and<T>(self, other: T) -> Update
    where
        T: Into<Update>,
    {
        Update::from(self).and(other)
    }
}

impl fmt::Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ADD ")?;

        let mut first = true;
        self.actions.iter().try_for_each(|action| {
            if first {
                first = false;
            } else {
                f.write_str(", ")?;
            }

            action.fmt(f)
        })
    }
}

impl From<AddAction> for Add {
    fn from(action: AddAction) -> Self {
        Self {
            actions: vec![action],
        }
    }
}

/// Represents an [`ADD` statement][1] in a [DynamoDB update expression][2].
///
/// Prefer [`Path::add`] over this.
///
/// See also: [`Add`], [`Update`], [`Set`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.ADD
/// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html
#[must_use = "Use in an update expression with `Update::from(add)`"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddAction {
    pub(crate) path: Path,
    pub(crate) value: ValueOrRef,
}

impl AddAction {
    /// Creates an [`AddAction`] for the specified [`Path`] and value.
    ///
    /// Prefer [`Path::add`] over this.
    pub fn new<N, V>(path: N, value: V) -> Self
    where
        N: Into<Path>,
        V: Into<AddValue>,
    {
        Self {
            path: path.into(),
            value: match value.into() {
                AddValue::Num(num) => Value::Scalar(num.into()).into(),
                AddValue::Set(set) => set.into(),
                AddValue::Ref(value_ref) => value_ref.into(),
            },
        }
    }

    /// Add an additional [`Update`] statement to this `ADD` statement.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// use dynamodb_expression::{update::AddAction, Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let update = AddAction::new("foo".parse::<Path>()?, Num::new(7))
    ///     .and("bar".parse::<Path>()?.set("a value"))
    ///     .and("baz".parse::<Path>()?.remove());
    /// assert_eq!(r#"SET bar = "a value" REMOVE baz ADD foo 7"#, update.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn and<T>(self, other: T) -> Update
    where
        T: Into<Update>,
    {
        Update::from(self).and(other)
    }
}

impl fmt::Display for AddAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.path.fmt(f)?;
        f.write_char(' ')?;
        self.value.fmt(f)
    }
}

/// A value that can be used for the [`ADD` operation][1] in a DynamoDB update expression.
///
/// See also: [`Path::add`], [`Add`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.ADD
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddValue {
    Set(Set),
    Num(Num),
    Ref(Ref),
}

impl fmt::Display for AddValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Set(value) => value.fmt(f),
            Self::Num(value) => value.fmt(f),
            Self::Ref(value) => value.fmt(f),
        }
    }
}

impl From<Set> for AddValue {
    fn from(value: Set) -> Self {
        Self::Set(value)
    }
}

impl From<StringSet> for AddValue {
    fn from(value: StringSet) -> Self {
        Self::Set(value.into())
    }
}

impl From<NumSet> for AddValue {
    fn from(value: NumSet) -> Self {
        Self::Set(value.into())
    }
}

impl From<BinarySet> for AddValue {
    fn from(value: BinarySet) -> Self {
        Self::Set(value.into())
    }
}

impl From<Num> for AddValue {
    fn from(value: Num) -> Self {
        Self::Num(value)
    }
}

impl From<Ref> for AddValue {
    fn from(value: Ref) -> Self {
        Self::Ref(value)
    }
}
