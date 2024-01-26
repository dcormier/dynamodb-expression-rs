use core::fmt;

use crate::{
    path::Path,
    update::Update,
    value::{Value, ValueOrRef},
};

/// Represents assigning a value of an [attribute][1], [list][2], or [map][3]
/// for a DynamoDB update expression.
///
/// Prefer [`Path::set`] over this.
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
    /// Allows for manual creation of an [`Assign`] statement.
    ///
    /// Prefer [`Path::set`] over this.
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

    /// Add an additional [`Update`] to this expression.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let set = "foo"
    ///     .parse::<Path>()?
    ///     .set(Num::new(7))
    ///     .and("bar".parse::<Path>()?.set("a value"))
    ///     .and("baz".parse::<Path>()?.remove());
    /// assert_eq!(r#"SET foo = 7, bar = "a value" REMOVE baz"#, set.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn and<T>(self, other: T) -> Update
    where
        T: Into<Update>,
    {
        Update::from(self).and(other)
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
    fn and() -> Result<(), Box<dyn std::error::Error>> {
        let assign: Assign = "foo".parse::<Path>()?.set("a value");
        let if_not_exists: IfNotExists = "bar".parse::<Path>()?.if_not_exists().set(Num::new(8));

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
            SetAction::from("baz".parse::<Path>()?.math().add(1)),
        ]
        .into_iter()
        .collect();
        let combined = assign.clone().and(set);
        assert_eq!(
            r#"SET foo = "a value", bar = if_not_exists(bar, 8), baz = baz + 1"#,
            combined.to_string()
        );

        // Should be able to concatenate a Remove instance

        let combined = assign.clone().and("quux".parse::<Path>()?.remove());
        assert_eq!(r#"SET foo = "a value" REMOVE quux"#, combined.to_string());

        // Should be able to concatenate a SetRemove instance

        let combined = assign.and("quux".parse::<Path>()?.remove());
        assert_eq!(r#"SET foo = "a value" REMOVE quux"#, combined.to_string());

        Ok(())
    }
}
