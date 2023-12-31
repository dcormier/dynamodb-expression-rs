use core::fmt::{self, Write};

use crate::path::Path;

/// The [DynamoDB `attribute_type` function][1]. True if the attribute at
/// the specified [`Path`] is of the specified data type.
///
/// See also: [`Path::attribute_type`], [Type]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeType {
    // `Path` is correct here
    // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Syntax
    pub(crate) path: Path,
    pub(crate) attribute_type: Type,
}

impl AttributeType {
    pub fn new<P>(path: P, attribute_type: Type) -> Self
    where
        P: Into<Path>,
    {
        Self {
            path: path.into(),
            attribute_type,
        }
    }
}

impl fmt::Display for AttributeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("attribute_type(")?;
        self.path.fmt(f)?;
        f.write_str(", ")?;
        self.attribute_type.fmt(f)?;
        f.write_char(')')
    }
}

/// The type of an attribute for the DynamoDB `attribute_type` function.
///
/// See also: [Path::attribute_type]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Type {
    String,
    StringSet,
    Number,
    NumberSet,
    Binary,
    BinarySet,
    Boolean,
    Null,
    List,
    Map,
}

impl Type {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::String => "S",
            Self::StringSet => "SS",
            Self::Number => "N",
            Self::NumberSet => "NS",
            Self::Binary => "B",
            Self::BinarySet => "BS",
            Self::Boolean => "BOOL",
            Self::Null => "NULL",
            Self::List => "L",
            Self::Map => "M",
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_str_eq;

    use super::Type::*;

    #[test]
    fn display_attribute_type() {
        assert_str_eq!("S", String.to_string());
        assert_str_eq!("SS", StringSet.to_string());
        assert_str_eq!("N", Number.to_string());
        assert_str_eq!("NS", NumberSet.to_string());
        assert_str_eq!("B", Binary.to_string());
        assert_str_eq!("BS", BinarySet.to_string());
        assert_str_eq!("BOOL", Boolean.to_string());
        assert_str_eq!("NULL", Null.to_string());
        assert_str_eq!("L", List.to_string());
        assert_str_eq!("M", Map.to_string());
    }
}
