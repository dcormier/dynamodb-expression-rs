use core::fmt;
use std::{convert::Infallible, str::FromStr};

use crate::{
    condition::{
        attribute_type::Type, AttributeType, BeginsWith, Between, Comparator, Comparison,
        Condition, Contains, In,
    },
    operand::{Operand, Size},
    update::{
        add::AddValue,
        set::{
            if_not_exists::Builder as IfNotExistsBuilder,
            list_append::Builder as ListAppendBuilder, math::Builder as MathBuilder,
        },
        Add, Assign, Delete, IfNotExists, ListAppend, Math, Remove,
    },
    value::{self, Scalar, Value},
};

/// Represents a DynamoDB [attribute name][1]. This will most commonly be used
/// for [top-level attributes][2].
///
/// Anything that can be turned into a `Name` can be turned into a [`Path`].
///
/// When used in an [`Expression`], attribute `Name`s are
/// automatically handled as [expression attribute names][3], allowing for names
/// that would not otherwise be permitted by DynamoDB. For example, `foo` would
/// become something similar to `#0` in the expression, and the name would be in
/// the `expression_attribute_names`.
///
/// ```
/// use dynamodb_expression::{Name, name};
///
/// // The `name()` function will turn anything that's `Into<String>` into a `Name`.
/// let name: Name = name("foo");
///
/// // A variety of strings can be turned into a `Name`.
/// let name: Name = "foo".into();
/// let name: Name = String::from("foo").into();
/// let name: Name = (&String::from("foo")).into();
/// let name: Name = (&"foo").into();
///
/// // `Name` also implements `FromStr`, so `parse()` can be used.
/// let name: Name = "foo".parse().unwrap();
/// ```
///
/// `Name` and `Path` can be converted between each other.
/// ```
/// use dynamodb_expression::{Name, path::Path};
///
/// // A `Name` can be converted into a `Path`
/// let name = Name::from("foo");
/// let path = Path::from(name);
/// assert_eq!(Path::from("foo"), path);
///
/// // A `Path` consisting of a single, unindexed field can be converted into a `Name`.
/// let path = Path::from("foo");
/// let name = Name::try_from(path).unwrap();
/// assert_eq!(Name::from("foo"), name);
///
/// // If the `Path` has more elements, or has indexes, it cannot be converted
/// // and the original `Path` is returned.
/// let path: Path = "foo[0]".parse().unwrap();
/// let err = Name::try_from(path.clone()).unwrap_err();
/// assert_eq!(path, err);
///
/// let path: Path = "foo.bar".parse().unwrap();
/// let err = Name::try_from(path.clone()).unwrap_err();
/// assert_eq!(path, err);
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html
/// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.TopLevelAttributes
/// [3]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeNames.html
/// [`parse()`]: str::parse
/// [`Expression`]: crate::expression::Expression
/// [`Path`]: crate::path::Path
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name {
    pub(crate) name: String,
}

impl Name {
    /// Compare two values.
    ///
    /// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators)
    // TODO: Operator-specific methods instead of this.
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
        T: Into<String>,
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
        V: Into<Scalar>,
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

// This block is for methods related to update expressions.
impl Name {
    /// For building an update expression. See [`Assign`].
    pub fn assign<T>(self, value: T) -> Assign
    where
        T: Into<Value>,
    {
        Assign::new(self, value)
    }

    /// Sets this as the destination in a [`Math`] builder for an update expression.
    pub fn math(self) -> MathBuilder {
        Math::builder(self)
    }

    /// Sets this as the destination in a [`ListAppend`] builder for an update expression.
    pub fn list_append(self) -> ListAppendBuilder {
        ListAppend::builder(self)
    }

    /// Sets this as the destination in an [`IfNotExists`] builder for an update expression.
    pub fn if_not_exists(self) -> IfNotExistsBuilder {
        IfNotExists::builder(self)
    }

    /// For building an update expression. See [`Delete`].
    pub fn delete<T>(self, set: T) -> Delete
    where
        T: Into<value::Set>,
    {
        Delete::new(self, set)
    }

    /// For building an update expression. See [`Add`].
    #[allow(clippy::should_implement_trait)]
    pub fn add<T>(self, value: T) -> Add
    where
        T: Into<AddValue>,
    {
        Add::new(self, value)
    }

    /// For building an update expression. See [`Remove`].
    ///
    /// [`Remove`]: crate::update::Remove
    pub fn remove(self) -> Remove {
        self.into()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl FromStr for Name {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

// // This would be ideal, but I think trait specialization is needed for this
// // to be workable without causing problems for things that want to do
// // `impl<T: Into<Name>> From<T> for OtherType`.
// impl<T> From<T> for Name
// where
//     T: Into<String>,
// {
//     fn from(name: T) -> Self {
//         Self { name: name.into() }
//     }
// }

impl From<String> for Name {
    fn from(name: String) -> Self {
        Self { name }
    }
}

impl From<&String> for Name {
    fn from(name: &String) -> Self {
        Self::from(name.to_owned())
    }
}

impl From<&str> for Name {
    fn from(name: &str) -> Self {
        Self::from(name.to_owned())
    }
}

impl From<&&str> for Name {
    fn from(name: &&str) -> Self {
        Self::from(name.to_owned())
    }
}

/// A convenience function for creating a [`Name`] instance.
pub fn name<T>(name: T) -> Name
where
    T: Into<String>,
{
    name.into().into()
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_str_eq;

    use crate::{num_value, Comparator};

    use super::name;

    #[test]
    fn size() {
        assert_str_eq!(
            "size(a) = 0",
            name("a")
                .size()
                .comparison(Comparator::Eq, num_value(0))
                .to_string()
        );
    }
}
