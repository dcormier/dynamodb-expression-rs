use core::fmt;

use crate::{
    path::Path,
    value::{Value, ValueOrRef},
};

use super::Set;

/// Represents assigning a value of an [attribute][1], [list][2], or [map][3]
/// for a DynamoDB update expression.
///
/// See also: [`Path::assign`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.ModifyingAttributes
/// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.AddingListElements
/// [3]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.AddingNestedMapAttributes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assign {
    pub(crate) path: Path,
    pub(crate) value: ValueOrRef,
}

impl Assign {
    pub fn new<P, V>(path: P, value: V) -> Self
    where
        P: Into<Path>,
        V: Into<Value>,
    {
        Self {
            path: path.into(),
            value: value.into().into(),
        }
    }

    /// Add an additional action to this `SET` statement.
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let set = Path::new_name("foo").assign(Num::new(7))
    ///     .and(Path::new_name("bar").assign("a value"));
    /// assert_eq!(r#"SET foo = 7, bar = "a value""#, set.to_string());
    /// ```
    pub fn and<T>(self, action: T) -> Set
    where
        T: Into<Set>,
    {
        Set::from(self).and(action)
    }
}

impl fmt::Display for Assign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.path.fmt(f)?;
        f.write_str(" = ")?;
        self.value.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::{
        update::{IfNotExists, Set, SetAction},
        Num, Path,
    };

    use super::Assign;

    #[test]
    fn and() {
        let assign: Assign = Path::new_name("foo").assign("a value");
        let if_not_exists: IfNotExists = Path::new_name("bar").if_not_exists().assign(Num::new(8));

        // Should be able to concatenate anything that can be turned into a SetAction.

        let combined = assign.clone().and(if_not_exists.clone());
        assert_eq!(
            r#"SET foo = "a value", bar = if_not_exists(bar, 8)"#,
            combined.to_string()
        );

        // Should be able to concatenate a SetAction instance.

        let combined = assign.clone().and(SetAction::from(if_not_exists.clone()));
        assert_eq!(
            r#"SET foo = "a value", bar = if_not_exists(bar, 8)"#,
            combined.to_string()
        );

        // Should be able to concatenate a Set instance

        let set: Set = if_not_exists.and(Path::new_name("baz").math().add(1));
        let combined = assign.and(set);
        assert_eq!(
            r#"SET foo = "a value", bar = if_not_exists(bar, 8), baz = baz + 1"#,
            combined.to_string()
        );
    }
}
