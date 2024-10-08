//! Types related to [DynamoDB key condition expressions][1].
//!
//! [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Query.KeyConditionExpressions.html

use core::fmt;

use crate::{
    condition::{
        equal, greater_than, greater_than_or_equal, less_than, less_than_or_equal, Condition,
    },
    operand::Operand,
    path::Path,
    value::StringOrRef,
};

/// Represents a [DynamoDB key condition expression][1].
///
/// An instance can be constructed using the [`Path::key`] method, or the
/// the `From<T: Into<Path>>` implementation.
///
/// See also: [`Path::key`]
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use dynamodb_expression::{key::Key, Path};
/// # use pretty_assertions::assert_eq;
///
/// let key: Key = "foo".parse::<Path>()?.key();
/// let key: Key = "foo".parse::<Path>()?.into();
/// #
/// # Ok(())
/// # }
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Query.KeyConditionExpressions.html
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key {
    // TODO: Is `Path` the right thing, here?
    //       Probably not. Looks like it should be `Name`
    //
    //       > Furthermore, each primary key attribute must be defined as type string, number, or binary.
    //
    //       https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes
    path: Path,
}

impl Key {
    /// The [DynamoDB `begins_with` function][1]. True if the attribute specified by
    ///  the [`Path`] begins with a particular substring.
    ///
    /// `begins_with` can take a string or a reference to an extended attribute
    /// value. Here's an example.
    ///
    /// See also: [`Ref`]
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use dynamodb_expression::{condition::BeginsWith, value::Ref, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let key_condition = "foo".parse::<Path>()?.key().begins_with("T");
    /// assert_eq!(r#"begins_with(foo, "T")"#, key_condition.to_string());
    ///
    /// let key_condition = "foo".parse::<Path>()?.key().begins_with(Ref::new("prefix"));
    /// assert_eq!(r#"begins_with(foo, :prefix)"#, key_condition.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions
    /// [`Ref`]: crate::value::Ref
    pub fn begins_with<T>(self, prefix: T) -> KeyCondition
    where
        T: Into<StringOrRef>,
    {
        KeyCondition {
            condition: self.path.begins_with(prefix),
        }
    }

    /// The [DynamoDB `BETWEEN` operator][1]. True if `self` is greater than or
    /// equal to `lower`, and less than or equal to `upper`.
    ///
    /// See also: [`Path::between`]
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let key_condition = "age"
    ///     .parse::<Path>()?
    ///     .key()
    ///     .between(Num::new(10), Num::new(90));
    /// assert_eq!(r#"age BETWEEN 10 AND 90"#, key_condition.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Comparators
    /// [`Path::between`]: crate::path::Path::between
    pub fn between<L, U>(self, lower: L, upper: U) -> KeyCondition
    where
        L: Into<Operand>,
        U: Into<Operand>,
    {
        KeyCondition {
            condition: self.path.between(lower, upper),
        }
    }

    /// A simple comparison that the specified attribute is equal to the
    /// provided value.
    pub fn equal<T>(self, right: T) -> KeyCondition
    where
        T: Into<Operand>,
    {
        KeyCondition {
            condition: equal(self.path, right).into(),
        }
    }

    /// A simple comparison that the specified attribute is greater than the
    /// provided value.
    pub fn greater_than<T>(self, right: T) -> KeyCondition
    where
        T: Into<Operand>,
    {
        KeyCondition {
            condition: greater_than(self.path, right).into(),
        }
    }

    /// A simple comparison that the specified attribute is greater than or
    /// equal to the provided value.
    pub fn greater_than_or_equal<T>(self, right: T) -> KeyCondition
    where
        T: Into<Operand>,
    {
        KeyCondition {
            condition: greater_than_or_equal(self.path, right).into(),
        }
    }

    /// A simple comparison that the specified attribute is less than the
    /// provided value.
    pub fn less_than<T>(self, right: T) -> KeyCondition
    where
        T: Into<Operand>,
    {
        KeyCondition {
            condition: less_than(self.path, right).into(),
        }
    }

    /// A simple comparison that the specified attribute is less than or
    /// equal to the provided value.
    pub fn less_than_or_equal<T>(self, right: T) -> KeyCondition
    where
        T: Into<Operand>,
    {
        KeyCondition {
            condition: less_than_or_equal(self.path, right).into(),
        }
    }
}

impl<T> From<T> for Key
where
    T: Into<Path>,
{
    /// Convert something that implements `Into<Path>` into a [`Key`].
    fn from(path: T) -> Self {
        Self { path: path.into() }
    }
}

/// Represents a DynamoDB [key condition expression][1]. Build an instance from
/// the methods on [`Key`].
///
/// See also: [`Path::key`], [`expression::Builder::with_key_condition`]
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use dynamodb_expression::{Expression, Num, Path};
///
/// let key_condition = "id"
///     .parse::<Path>()?
///     .key()
///     .equal(Num::new(42))
///     .and("category".parse::<Path>()?.key().begins_with("hardware."));
///
/// let expression = Expression::builder().with_key_condition(key_condition).build();
/// # _ = expression;
/// #
/// # Ok(())
/// # }
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Query.KeyConditionExpressions.html
/// [`expression::Builder::with_key_condition`]: crate::expression::Builder::with_key_condition
#[must_use = "Use in a DynamoDB expression with \
    `Expression::builder().with_key_condition(key_condition)`"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyCondition {
    pub(crate) condition: Condition,
}

impl KeyCondition {
    /// Combine two [`KeyCondition`]s with the `AND` operator.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let key_condition = "id"
    ///     .parse::<Path>()?
    ///     .key()
    ///     .equal(Num::new(42))
    ///     .and("category".parse::<Path>()?.key().begins_with("hardware."));
    /// assert_eq!(r#"id = 42 AND begins_with(category, "hardware.")"#, key_condition.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn and(self, right: Self) -> Self {
        Self {
            condition: self.condition.and(right.condition),
        }
    }
}

impl fmt::Display for KeyCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.condition.fmt(f)
    }
}

impl From<KeyCondition> for String {
    fn from(key_condition: KeyCondition) -> Self {
        key_condition.condition.into()
    }
}

#[cfg(test)]
mod test {
    use crate::{value::Ref, Path};

    use super::Key;

    #[test]
    fn begins_with_string() {
        // Begins with &str
        let begins_with = Key::from("foo".parse::<Path>().unwrap()).begins_with("foo");
        assert_eq!(r#"begins_with(foo, "foo")"#, begins_with.to_string());

        // Begins with String
        let begins_with =
            Key::from("foo".parse::<Path>().unwrap()).begins_with(String::from("foo"));
        assert_eq!(r#"begins_with(foo, "foo")"#, begins_with.to_string());

        // Begins with &String
        #[expect(
            clippy::needless_borrows_for_generic_args,
            reason = "Explicitly testing &String"
        )]
        let begins_with =
            Key::from("foo".parse::<Path>().unwrap()).begins_with(&String::from("foo"));
        assert_eq!(r#"begins_with(foo, "foo")"#, begins_with.to_string());

        // Begins with &&str
        #[expect(
            clippy::needless_borrows_for_generic_args,
            reason = "Explicitly testing &&str"
        )]
        let begins_with = Key::from("foo".parse::<Path>().unwrap()).begins_with(&"foo");
        assert_eq!(r#"begins_with(foo, "foo")"#, begins_with.to_string());
    }

    #[test]
    fn begins_with_value_ref() {
        let begins_with = Key::from("foo".parse::<Path>().unwrap()).begins_with(Ref::new("prefix"));
        assert_eq!("begins_with(foo, :prefix)", begins_with.to_string());
    }
}
