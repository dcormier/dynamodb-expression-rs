use core::fmt;

use crate::update::set_remove::SetRemove;

use super::{Assign, IfNotExists, ListAppend, Math, Set};

/// Represents an action to take in a [`SET` statement][1] for an update expression.
///
/// See also: [`Set`], [`Update`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET
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
    /// Add an additional [`Set`] or [`Remove`] statement to this expression.
    ///
    /// ```
    /// use dynamodb_expression::{Num, Path, update::SetAction};
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
    use pretty_assertions::assert_eq;

    use crate::{
        update::{set_remove::SetRemove, Assign, IfNotExists, ListAppend, Math, Set},
        Num, Path,
    };

    use super::SetAction;

    #[test]
    fn from() {
        let assign: Assign = Path::new_name("foo").set(Num::new(8));
        let if_not_exists: IfNotExists = Path::new_name("bar").if_not_exists().set(Num::new(7));
        let math: Math = Path::new_name("baz").math().add(1);
        let list_append: ListAppend = Path::new_name("quux").list_append().list(["d", "e", "f"]);

        let _set_actions = [
            SetAction::from(assign),
            SetAction::from(if_not_exists),
            SetAction::from(math),
            SetAction::from(list_append),
        ];
    }

    #[test]
    fn and() {
        let assign: Assign = Path::new_name("bar").set(Num::new(8));
        let set_action: SetAction = Path::new_name("foo").set("a value").into();

        // Should be able to concatenate anything that can be turned into a SetAction.

        let combined = set_action.clone().and(assign.clone());
        assert_eq!(r#"SET foo = "a value", bar = 8"#, combined.to_string());

        // Should be able to concatenate a SetAction instance.

        let combined = set_action.clone().and(SetAction::from(assign.clone()));
        assert_eq!(r#"SET foo = "a value", bar = 8"#, combined.to_string());

        // Should be able to concatenate a Set instance

        let set: Set = [
            SetAction::from(assign),
            SetAction::from(Path::new_name("baz").if_not_exists().set(Num::new(7))),
        ]
        .into_iter()
        .collect();
        let combined = set_action.clone().and(set);
        assert_eq!(
            r#"SET foo = "a value", bar = 8, baz = if_not_exists(baz, 7)"#,
            combined.to_string()
        );

        // Should be able to concatenate a Remove instance

        let combined = set_action.clone().and(Path::new_name("quux").remove());
        assert_eq!(r#"SET foo = "a value" REMOVE quux"#, combined.to_string());

        // Should be able to concatenate a SetRemove instance

        let combined = set_action.and(SetRemove::from(Path::new_name("quux").remove()));
        assert_eq!(r#"SET foo = "a value" REMOVE quux"#, combined.to_string());
    }
}
