use core::fmt;

use crate::{
    path::Path,
    value::{Num, Ref, Set, Value, ValueOrRef},
};

/// Represents an [`ADD` statement][1] in a [DynamoDB update expression][2].
///
/// The [DynamoDB documentation recommends][1] against using `ADD`:
///
/// > In general, we recommend using `SET` rather than `ADD`.
///
/// See also: [`Update`], [`Set`].
///
/// # Examples
///
/// ```
/// use dynamodb_expression::{path::{Name, Path}, update::Add, value::Num};
///
/// let update = Add::new(Name::from("foo"), Num::from(1));
/// assert_eq!("ADD foo 1", update.to_string());
///
/// let update = Add::new("foo[4]".parse::<Path>().unwrap(), Num::from(1));
/// assert_eq!("ADD foo[4] 1", update.to_string());
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.ADD
/// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html
/// [`Update`]: crate::update::Update
/// [`Set`]: crate::update::Set
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Add {
    pub(crate) path: Path,
    pub(crate) value: ValueOrRef,
}

impl Add {
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
}

impl fmt::Display for Add {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD {} {}", self.path, self.value)
    }
}

/// A value that can be used for the `ADD` operation in a DynamoDB update request.
///
/// See also: [`Add`]
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
