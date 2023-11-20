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
/// use dynamodb_expression::{key::Key, Path};
/// # use pretty_assertions::assert_eq;
///
/// let key: Key = Path::new_name("foo").key();
/// let key: Key = Path::new_name("foo").into();
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
    /// use dynamodb_expression::{condition::BeginsWith, value::Ref, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let begins_with = Path::new_name("foo").key().begins_with("T");
    /// assert_eq!(r#"begins_with(foo, "T")"#, begins_with.to_string());
    ///
    /// let begins_with = Path::new_name("foo").key().begins_with(Ref::new("prefix"));
    /// assert_eq!(r#"begins_with(foo, :prefix)"#, begins_with.to_string());
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
    /// use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let key_condition = Path::new_name("age")
    ///     .key()
    ///     .between(Num::new(10), Num::new(90));
    /// assert_eq!(r#"age BETWEEN 10 AND 90"#, key_condition.to_string());
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

/// Represents a DynamoDB [key condition expression].
///
/// See also: [`Key`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Query.KeyConditionExpressions.html
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyCondition {
    pub(crate) condition: Condition,
}

impl KeyCondition {
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
        let begins_with = Key::from(Path::new_name("foo")).begins_with("foo");
        assert_eq!(r#"begins_with(foo, "foo")"#, begins_with.to_string());

        let begins_with = Key::from(Path::new_name("foo")).begins_with(String::from("foo"));
        assert_eq!(r#"begins_with(foo, "foo")"#, begins_with.to_string());

        #[allow(clippy::needless_borrows_for_generic_args)]
        let begins_with = Key::from(Path::new_name("foo")).begins_with(&String::from("foo"));
        assert_eq!(r#"begins_with(foo, "foo")"#, begins_with.to_string());

        #[allow(clippy::needless_borrows_for_generic_args)]
        let begins_with = Key::from(Path::new_name("foo")).begins_with(&"foo");
        assert_eq!(r#"begins_with(foo, "foo")"#, begins_with.to_string());
    }

    #[test]
    fn begins_with_value_ref() {
        let begins_with = Key::from(Path::new_name("foo")).begins_with(Ref::new("prefix"));
        assert_eq!("begins_with(foo, :prefix)", begins_with.to_string());
    }
}
