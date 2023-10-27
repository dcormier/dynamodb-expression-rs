use core::fmt::{self, Write};

use super::Value;

/// A DynamoDB value, or a reference to one stored in the collected expression values
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum ValueOrRef {
    Value(Value),
    Ref(Ref),
}

impl fmt::Display for ValueOrRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueOrRef::Value(value) => value.fmt(f),
            ValueOrRef::Ref(value) => value.fmt(f),
        }
    }
}

impl<T> From<T> for ValueOrRef
where
    T: Into<Value>,
{
    fn from(value: T) -> Self {
        Self::Value(value.into())
    }
}

impl From<Ref> for ValueOrRef {
    fn from(value: Ref) -> Self {
        Self::Ref(value)
    }
}

/// A reference to a DynamoDB value stored in expression attribute values.
/// Automatically prefixed with `:`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ref(String);

impl<T> From<T> for Ref
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl fmt::Display for Ref {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(':')?;
        self.0.fmt(f)
    }
}

/// A reference to a DynamoDB value stored in expression attribute values.
/// Automatically prefixed with `:`.
pub fn ref_value<T>(value: T) -> Ref
where
    T: Into<String>,
{
    Ref::from(value)
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_str_eq;

    use super::{Ref, ValueOrRef};
    use crate::string_value;

    #[test]
    fn display_value() {
        let vr = ValueOrRef::from(string_value("foo"));
        assert_str_eq!("\"foo\"", vr.to_string());
    }

    #[test]
    fn display_ref() {
        let vr = ValueOrRef::Ref(Ref("foo".into()));
        assert_str_eq!(":foo", vr.to_string());
    }
}
