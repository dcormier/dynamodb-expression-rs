use core::fmt::{self, Display};

use crate::{attribute_type::AttributeType, expression::Expression};

/**
[DynamoDB functions](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)

See [the functions in this module] to create [`Expression`]s for DynamoDB functions directly.

```no-compile
function ::=
    attribute_exists (path)
    | attribute_not_exists (path)
    | attribute_type (path, type)
    | begins_with (path, substr)
    | contains (path, operand)
    | size (path)
```

[the functions in this module]: self#functions
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Function {
    AttributeExists(String),
    AttributeNotExists(String),
    AttributeType(String, AttributeType),
    BeginsWith(String, String),
    Contains(String, String),
    Size(String),
}

impl Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Self::AttributeExists(path) => format!("attribute_exists({path})"),
            Self::AttributeNotExists(path) => format!("attribute_not_exists({path})"),
            Self::AttributeType(path, attr_type) => format!("attribute_type({path}, {attr_type})"),
            Self::BeginsWith(path, substr) => format!("begins_with({path}, {substr})"),
            Self::Contains(path, operand) => format!("contains({path}, {operand})"),
            Self::Size(path) => format!("size({path})"),
        };

        f.write_str(&str)
    }
}

/// True if the item contains the attribute specified by `path`.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
pub fn attribute_exists<P>(path: P) -> Expression
where
    P: Into<String>,
{
    Function::AttributeExists(path.into()).into()
}

/// True if the attribute specified by `path` does not exist in the item.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
pub fn attribute_not_exists<P>(path: P) -> Expression
where
    P: Into<String>,
{
    Function::AttributeNotExists(path.into()).into()
}

/// True if the attribute at the specified `path` is of a particular data type.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
pub fn attribute_type<P>(path: P, attribute_type: AttributeType) -> Expression
where
    P: Into<String>,
{
    Function::AttributeType(path.into(), attribute_type).into()
}

/// True if the attribute specified by `path` begins with a particular substring.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
pub fn begins_with<P, S>(path: P, substr: S) -> Expression
where
    P: Into<String>,
    S: Into<String>,
{
    Function::BeginsWith(path.into(), substr.into()).into()
}

/// True if the attribute specified by `path` is one of the following:
/// * A `String` that contains a particular substring.
/// * A `Set` that contains a particular element within the set.
/// * A `List` that contains a particular element within the list.
///
/// The operand must be a `String` if the attribute specified by path is a `String`.
/// If the attribute specified by path is a `Set`, the operand must be the set's element type.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
pub fn contains<P, S>(path: P, operand: S) -> Expression
where
    P: Into<String>,
    S: Into<String>,
{
    Function::Contains(path.into(), operand.into()).into()
}

/// Returns a number representing an attribute's size.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
pub fn size<P>(path: P) -> Expression
where
    P: Into<String>,
{
    Function::Size(path.into()).into()
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_str_eq;

    use crate::attribute_type::AttributeType::*;

    use super::*;

    #[test]
    fn display_function() {
        assert_str_eq!("attribute_exists(foo)", attribute_exists("foo").to_string());
        assert_str_eq!("attribute_exists(bar)", attribute_exists("bar").to_string());
        assert_str_eq!(
            "attribute_not_exists(foo)",
            attribute_not_exists("foo").to_string()
        );
        assert_str_eq!(
            "attribute_not_exists(bar)",
            attribute_not_exists("bar").to_string()
        );
        assert_str_eq!(
            "attribute_type(foo, S)",
            attribute_type("foo", String).to_string()
        );
        assert_str_eq!(
            "attribute_type(bar, SS)",
            attribute_type("bar", StringSet).to_string()
        );
        assert_str_eq!(
            "begins_with(foo, bar)",
            begins_with("foo", "bar").to_string()
        );
        assert_str_eq!(
            "begins_with(bar, baz)",
            begins_with("bar", "baz").to_string()
        );
        assert_str_eq!("contains(foo, bar)", contains("foo", "bar").to_string());
        assert_str_eq!("contains(bar, baz)", contains("bar", "baz").to_string());
        assert_str_eq!("size(foo)", size("foo").to_string());
        assert_str_eq!("size(bar)", size("bar").to_string());
    }
}
