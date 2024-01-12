use core::fmt;

use crate::path::Path;

use super::set_remove::SetRemove;

/// For use an in an update expression to [remove attributes from an
/// item][1], or [elements from a list][2].
///
/// See also: [`Path::remove`], [`SetRemove`], [`Update`]
///
/// # Examples
///
/// ```
/// use dynamodb_expression::{Expression, Path, update::{Remove, Update}};
/// # use pretty_assertions::assert_eq;
///
/// let remove = "foo".parse::<Path>().unwrap().remove();
/// assert_eq!("REMOVE foo", remove.to_string());
///
/// let remove = Remove::from("foo[8]".parse::<Path>().unwrap());
/// assert_eq!("REMOVE foo[8]", remove.to_string());
///
/// let remove: Remove = ["foo", "bar", "baz"].into_iter().map(Path::new_name).collect();
/// assert_eq!("REMOVE foo, bar, baz", remove.to_string());
///
/// let remove = remove.and(Path::new_name("quux").remove());
/// assert_eq!("REMOVE foo, bar, baz, quux", remove.to_string());
///
/// // Use in an update expression
/// let update = Update::from(remove.clone());
///
/// // Use an expression builder
/// let expression = Expression::builder().with_update(remove).build();
/// # _ = expression;
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.REMOVE
/// [2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.REMOVE.RemovingListElements
/// [`SetRemove`]: crate::update::SetRemove
/// [`Update`]: crate::update::Update
#[must_use = "Use in an update expression with `Update::from(remove)`"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Remove {
    pub(crate) paths: Vec<Path>,
}

impl Remove {
    /// Add an additional [`Remove`] or [`Set`] statement to this expression.
    ///
    /// ```
    /// use dynamodb_expression::Path;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let remove = Path::new_name("foo").remove().and(Path::new_name("bar").remove());
    /// assert_eq!("REMOVE foo, bar", remove.to_string());
    ///
    /// let set_remove = remove.and(Path::new_name("baz").set("a value"));
    /// assert_eq!(r#"SET baz = "a value" REMOVE foo, bar"#, set_remove.to_string());
    /// ```
    ///
    /// [`Set`]: crate::update::Set
    pub fn and<T>(self, other: T) -> SetRemove
    where
        T: Into<SetRemove>,
    {
        SetRemove::from(self).and(other)
    }
}

impl fmt::Display for Remove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("REMOVE ")?;

        let mut first = true;
        self.paths.iter().try_for_each(|name| {
            if first {
                first = false;
            } else {
                f.write_str(", ")?;
            }

            name.fmt(f)
        })
    }
}

impl<T> From<T> for Remove
where
    T: Into<Path>,
{
    fn from(path: T) -> Self {
        Self {
            paths: vec![path.into()],
        }
    }
}

impl<T> FromIterator<T> for Remove
where
    T: Into<Path>,
{
    fn from_iter<I>(paths: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            paths: paths.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::Path;

    #[test]
    fn sub_attributes() {
        assert_eq!(
            "REMOVE foo.bar",
            "foo.bar".parse::<Path>().unwrap().remove().to_string()
        );
        assert_eq!(
            "REMOVE foo[3].bar",
            "foo[3].bar".parse::<Path>().unwrap().remove().to_string()
        );
        assert_eq!(
            "REMOVE foo[3][7]",
            "foo[3][7]".parse::<Path>().unwrap().remove().to_string()
        );
    }

    #[test]
    fn and() {
        let remove = Path::new_name("foo")
            .remove()
            .and(Path::new_name("bar").remove());
        assert_eq!("REMOVE foo, bar", remove.to_string());
    }
}
