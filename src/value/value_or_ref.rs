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

impl Ref {
    pub fn new<T>(value_ref: T) -> Self
    where
        T: Into<String>,
    {
        Self(value_ref.into())
    }
}

impl From<String> for Ref {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&String> for Ref {
    fn from(value: &String) -> Self {
        Self(value.to_owned())
    }
}

impl From<&str> for Ref {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<&&str> for Ref {
    fn from(value: &&str) -> Self {
        Self((*value).to_owned())
    }
}

impl fmt::Display for Ref {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(':')?;
        self.0.fmt(f)
    }
}

impl From<Ref> for String {
    fn from(mut value: Ref) -> Self {
        value.0.insert(0, ':');

        value.0
    }
}

/// Represents a value that is either a string, or a reference to a value
/// already in the expression attribute values.
pub enum StringOrRef {
    String(String),
    Ref(Ref),
}

impl From<String> for StringOrRef {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&String> for StringOrRef {
    fn from(value: &String) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<&str> for StringOrRef {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<&&str> for StringOrRef {
    fn from(value: &&str) -> Self {
        Self::String((*value).to_owned())
    }
}

impl From<Ref> for StringOrRef {
    fn from(value: Ref) -> Self {
        Self::Ref(value)
    }
}

impl From<StringOrRef> for ValueOrRef {
    fn from(value: StringOrRef) -> Self {
        match value {
            StringOrRef::String(value) => value.into(),
            StringOrRef::Ref(value) => value.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_str_eq;

    use crate::Scalar;

    use super::{Ref, ValueOrRef};

    #[test]
    fn display_value() {
        let vr = ValueOrRef::from(Scalar::new_string("foo"));
        assert_str_eq!("\"foo\"", vr.to_string());
    }

    #[test]
    fn display_ref() {
        let vr = ValueOrRef::Ref(Ref("foo".into()));
        assert_str_eq!(":foo", vr.to_string());
    }
}
