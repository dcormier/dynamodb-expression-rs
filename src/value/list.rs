use core::fmt::{self, Write};

use aws_sdk_dynamodb::types::AttributeValue;

use super::{Scalar, Value};

/// A collection of DynamoDB values that may not all be of the same type.
/// Represents a [DynamoDB list][1].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.Document.List
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct List {
    list: Vec<Value>,
}

impl List {
    pub fn new<T>(list: T) -> Self
    where
        T: Into<List>,
    {
        list.into()
    }

    // Intentionally not using `impl From<ScalarValue> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    pub(super) fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::L(
            self.list
                .into_iter()
                .map(Value::into_attribute_value)
                .collect(),
        )
    }
}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.list.iter()).finish()
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('[')?;

        let mut first = true;
        self.list.iter().try_for_each(|v| {
            if first {
                first = false;
            } else {
                f.write_str(", ")?;
            }

            if let Value::Scalar(Scalar::String(s)) = v {
                serde_json::to_string(s).unwrap().fmt(f)
            } else {
                v.fmt(f)
            }
        })?;

        f.write_char(']')
    }
}

impl<T> FromIterator<T> for List
where
    T: Into<Value>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            list: iter.into_iter().map(Into::into).collect(),
        }
    }
}

impl<I, T> From<I> for List
where
    I: IntoIterator<Item = T>,
    T: Into<Value>,
{
    fn from(iter: I) -> Self {
        Self::from_iter(iter)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_str_eq;

    use super::List;
    use crate::value::Value;

    #[test]
    fn display() {
        let list = List::from(["a"]);
        assert_str_eq!(r#"["a"]"#, list.to_string());

        let list = List::from([Value::new_string("a"), Value::new_num(42)]);
        assert_str_eq!(r#"["a", 42]"#, list.to_string());
    }
}
