use core::fmt;

use crate::{
    path::Path,
    value::{Value, ValueOrRef},
};

/// Represents an update expression to [set an attribute if it doesn't exist][1].
///
/// # Examples
///
/// ```
/// use dynamodb_expression::{num_value, path::{Name, Path}, update::IfNotExists};
/// # use pretty_assertions::assert_eq;
///
/// let if_not_exists = IfNotExists::builder(Name::from("foo")).value(num_value(7));
/// assert_eq!("foo = if_not_exists(foo, 7)", if_not_exists.to_string());
///
/// let if_not_exists_2 = Path::from(Name::from("foo")).if_not_exists().value(num_value(7));
/// assert_eq!(if_not_exists, if_not_exists_2);
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.PreventingAttributeOverwrites
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfNotExists {
    // TODO: Is `Path` the right thing, here?
    pub(crate) dst: Path,
    pub(crate) src: Option<Path>,
    pub(crate) value: ValueOrRef,
}

impl IfNotExists {
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

impl fmt::Display for IfNotExists {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { dst, src, value } = self;

        // If no source field is specified, default to using the destination.
        let src = src.as_ref().unwrap_or(dst);

        write!(f, "{dst} = if_not_exists({src}, {value})")
    }
}

/// Builds an [`IfNotExists`] instance. Create an instance of this by using [`IfNotExists::builder`].
#[must_use = "Consume the `Builder` using its `.value()` method"]
#[derive(Debug, Clone)]
pub struct Builder {
    dst: Path,
    src: Option<Path>,
}

impl Builder {
    /// Sets the source field check for existence. Defaults to the destination field.
    pub fn src<T>(mut self, src: T) -> Self
    where
        T: Into<Path>,
    {
        self.src = Some(src.into());

        self
    }

    /// The value to conditionally set. Builds the `IfNotExists` instance.
    pub fn value<T>(self, value: T) -> IfNotExists
    where
        T: Into<Value>,
    {
        let Self { dst, src } = self;

        IfNotExists {
            dst,
            src,
            value: value.into().into(),
        }
    }
}
