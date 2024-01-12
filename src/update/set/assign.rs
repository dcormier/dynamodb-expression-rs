use core::fmt;

use crate::{
    path::Path,
    update::set_remove::SetRemove,
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
#[must_use] // TODO: More detail
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

    /// Add an additional [`Set`] or [`Remove`] to this expression.
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let set = Path::new_name("foo")
    ///     .set(Num::new(7))
    ///     .and(Path::new_name("bar").set("a value"))
    ///     .and(Path::new_name("baz").remove());
    /// assert_eq!(r#"SET foo = 7, bar = "a value" REMOVE baz"#, set.to_string());
    /// ```
    ///
    /// [`Remove`]: crate::update::Remove
    pub fn and<T>(self, action: T) -> SetRemove
    where
        T: Into<SetRemove>,
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
        update::{set_remove::SetRemove, IfNotExists, Set, SetAction},
        Num, Path,
    };

    use super::Assign;

    #[test]
    fn and() {
        let assign: Assign = Path::new_name("foo").set("a value");
        let if_not_exists: IfNotExists = Path::new_name("bar").if_not_exists().set(Num::new(8));

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

        let set: Set = [
            SetAction::from(if_not_exists),
            SetAction::from(Path::new_name("baz").math().add(1)),
        ]
        .into_iter()
        .collect();
        let combined = assign.clone().and(set);
        assert_eq!(
            r#"SET foo = "a value", bar = if_not_exists(bar, 8), baz = baz + 1"#,
            combined.to_string()
        );

        // Should be able to concatenate a Remove instance

        let combined = assign.clone().and(Path::new_name("quux").remove());
        assert_eq!(r#"SET foo = "a value" REMOVE quux"#, combined.to_string());

        // Should be able to concatenate a SetRemove instance

        let combined = assign.and(SetRemove::from(Path::new_name("quux").remove()));
        assert_eq!(r#"SET foo = "a value" REMOVE quux"#, combined.to_string());
    }
}
