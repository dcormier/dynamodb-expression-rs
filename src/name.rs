use alloc::borrow::Cow;
use core::fmt;

use crate::{
    condition::{
        attribute_type::Type, AttributeType, BeginsWith, Between, Comparator, Comparison,
        Condition, Contains, In,
    },
    operand::{Operand, Size},
    ScalarValue,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name {
    pub name: Cow<'static, str>,
}

impl Name {
    /// Compare two values.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn comparison<R>(self, cmp: Comparator, right: R) -> Condition
    where
        R: Into<Operand>,
    {
        Condition::Comparison(Comparison {
            left: self.into(),
            cmp,
            right: right.into(),
        })
    }

    /// `self BETWEEN b AND c` - true if `self` is greater than or equal to `b`, and less than or equal to `c`.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn between<L, U>(self, lower: L, upper: U) -> Condition
    where
        L: Into<Operand>,
        U: Into<Operand>,
    {
        Condition::Between(Between {
            op: self.into(),
            lower: lower.into(),
            upper: upper.into(),
        })
    }

    /// `self IN (b[, ..])` — true if `self` is equal to any value in the list.
    ///
    /// The list can contain up to 100 values. It must have at least 1.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    pub fn in_<I, T>(self, items: I) -> Condition
    where
        I: IntoIterator<Item = T>,
        T: Into<Operand>,
    {
        In::new(self.into(), items).into()
    }

    /// True if the item contains the attribute specified by `path`.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
    pub fn attribute_exists(self) -> Condition {
        Condition::AttributeExists(self.into())
    }

    /// True if the attribute specified by `path` does not exist in the item.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
    pub fn attribute_not_exists(self) -> Condition {
        Condition::AttributeNotExists(self.into())
    }

    /// True if the attribute at the specified `path` is of a particular data type.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
    pub fn attribute_type(self, attribute_type: Type) -> Condition {
        AttributeType::new(self.name, attribute_type).into()
    }

    /// True if the attribute specified by `path` begins with a particular substring.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
    pub fn begins_with<T>(self, prefix: T) -> Condition
    where
        T: Into<Cow<'static, str>>,
    {
        BeginsWith::new(self.name, prefix).into()
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
    pub fn contains<V>(self, operand: V) -> Condition
    where
        V: Into<ScalarValue>,
    {
        Contains::new(self.name, operand).into()
    }

    /// Returns a number representing an attribute's size.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
    pub fn size(self) -> Size {
        self.into()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl<T> From<T> for Name
where
    T: Into<Cow<'static, str>>,
{
    fn from(name: T) -> Self {
        Self { name: name.into() }
    }
}

pub fn name<T>(name: T) -> Name
where
    T: Into<Name>,
{
    name.into()
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_str_eq;

    use crate::{condition::Comparator, string_value};

    use super::name;

    #[test]
    fn size() {
        assert_str_eq!(
            "size(a) = 0",
            name("a")
                .size()
                .comparison(Comparator::Eq, string_value("0"))
                .to_string()
        );
    }
}
