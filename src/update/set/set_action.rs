use core::fmt;

use crate::update::Update;

use super::{Assign, IfNotExists, ListAppend, Math};

/// Represents an action to take in a [`SET` statement][1] for an update expression.
///
/// See also: [`Set`], [`Update`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET
/// [`Set`]: crate::update::Set
/// [`Update`]: crate::update::Update
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetAction {
    /// Assign a value in a `SET` statement for an update expression.
    ///
    /// See also: [`Assign`]
    Assign(Assign),

    /// Perform [math against a value in a `SET` statement][1] for an update expression.
    ///
    /// See also: [`Math`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.IncrementAndDecrement
    Math(Math),

    /// [Add values to a list][1] in a `SET` statement for an update expression.
    ///
    /// See also: [`ListAppend`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.UpdatingListElements
    ListAppend(ListAppend),

    /// Assign a value [only if it doesn't exist][1].
    ///
    /// See also: [`IfNotExists`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.PreventingAttributeOverwrites
    IfNotExists(IfNotExists),
}

impl SetAction {
    /// Add an additional [`Update`] statement to this expression.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
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

impl From<Assign> for SetAction {
    fn from(assign: Assign) -> Self {
        Self::Assign(assign)
    }
}

impl From<Math> for SetAction {
    fn from(math: Math) -> Self {
        Self::Math(math)
    }
}

impl From<ListAppend> for SetAction {
    fn from(append: ListAppend) -> Self {
        Self::ListAppend(append)
    }
}

impl From<IfNotExists> for SetAction {
    fn from(if_not_exists: IfNotExists) -> Self {
        Self::IfNotExists(if_not_exists)
    }
}

impl fmt::Display for SetAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetAction::Assign(action) => action.fmt(f),
            SetAction::Math(action) => action.fmt(f),
            SetAction::ListAppend(action) => action.fmt(f),
            SetAction::IfNotExists(action) => action.fmt(f),
        }
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use pretty_assertions::assert_eq;

    use crate::{
        update::{Assign, IfNotExists, ListAppend, Math, Set},
        Num, Path,
    };

    use super::SetAction;

    #[test]
    fn from() -> Result<(), Box<dyn Error>> {
        let assign: Assign = "foo".parse::<Path>()?.set(Num::new(8));
        let if_not_exists: IfNotExists = "bar".parse::<Path>()?.if_not_exists().set(Num::new(7));
        let math: Math = "baz".parse::<Path>()?.math().add(1);
        let list_append: ListAppend = "quux".parse::<Path>()?.list_append().list(["d", "e", "f"]);

        let _set_actions = [
            SetAction::from(assign),
            SetAction::from(if_not_exists),
            SetAction::from(math),
            SetAction::from(list_append),
        ];

        Ok(())
    }

    #[test]
    fn and() -> Result<(), Box<dyn Error>> {
        let assign: Assign = "bar".parse::<Path>()?.set(Num::new(8));
        let set_action: SetAction = "foo".parse::<Path>()?.set("a value").into();

        // Should be able to concatenate anything that can be turned into a SetAction.

        let combined = set_action.clone().and(assign.clone());
        assert_eq!(r#"SET foo = "a value", bar = 8"#, combined.to_string());

        // Should be able to concatenate a SetAction instance.

        let combined = set_action.clone().and(SetAction::from(assign.clone()));
        assert_eq!(r#"SET foo = "a value", bar = 8"#, combined.to_string());

        // Should be able to concatenate a Set instance

        let set: Set = [
            SetAction::from(assign),
            SetAction::from("baz".parse::<Path>()?.if_not_exists().set(Num::new(7))),
        ]
        .into_iter()
        .collect();
        let combined = set_action.clone().and(set);
        assert_eq!(
            r#"SET foo = "a value", bar = 8, baz = if_not_exists(baz, 7)"#,
            combined.to_string()
        );

        // Should be able to concatenate a Remove instance

        let combined = set_action.clone().and("quux".parse::<Path>()?.remove());
        assert_eq!(r#"SET foo = "a value" REMOVE quux"#, combined.to_string());

        // Should be able to concatenate a SetRemove instance

        let combined = set_action.and("quux".parse::<Path>()?.remove());
        assert_eq!(r#"SET foo = "a value" REMOVE quux"#, combined.to_string());

        Ok(())
    }
}
