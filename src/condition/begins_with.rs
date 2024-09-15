use core::fmt::{self, Write};

use crate::{
    path::Path,
    value::{StringOrRef, ValueOrRef},
};

/// The [DynamoDB `begins_with` function][1]. True if the attribute specified by
///  the [`Path`] begins with a particular substring.
///
/// See also: [`Path::begins_with`], [`Key::begins_with`]
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use dynamodb_expression::{condition::BeginsWith, Path};
/// # use pretty_assertions::assert_eq;
///
/// let begins_with = "foo".parse::<Path>()?.begins_with("T");
/// assert_eq!(r#"begins_with(foo, "T")"#, begins_with.to_string());
///
/// let begins_with = "foo".parse::<Path>()?.key().begins_with("T");
/// assert_eq!(r#"begins_with(foo, "T")"#, begins_with.to_string());
/// #
/// # Ok(())
/// # }
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions
/// [`Key::begins_with`]: crate::key::Key::begins_with
/// [`Ref`]: crate::value::Ref
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeginsWith {
    // `Path` is correct here
    // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Syntax
    pub(crate) path: Path,
    pub(crate) substr: ValueOrRef,
}

impl BeginsWith {
    /// Allows for manually constructing a `BeginsWith` instance.
    ///
    /// The `substr` argument can be a string or a reference to an expression
    /// attribute value. Here's an example.
    ///
    /// See also: [`Path::begins_with`], [`Key::begins_with`], [`Ref`]
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use dynamodb_expression::{condition::BeginsWith, value::Ref, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let begins_with = BeginsWith::new("foo".parse::<Path>()?, "T");
    /// assert_eq!(r#"begins_with(foo, "T")"#, begins_with.to_string());
    ///
    /// let begins_with = BeginsWith::new("foo".parse::<Path>()?, Ref::new("prefix"));
    /// assert_eq!(r#"begins_with(foo, :prefix)"#, begins_with.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Key::begins_with`]: crate::key::Key::begins_with
    /// [`Ref`]: crate::value::Ref
    pub fn new<P, S>(path: P, substr: S) -> Self
    where
        P: Into<Path>,
        // Per the docs below, this can be a string or a reference to an expression attribute value.
        //
        // > True if the attribute specified by path begins with a particular substring.
        // >
        // > Example: Check whether the first few characters of the front view picture URL are http://.
        // >
        // > begins_with (Pictures.FrontView, :v_sub)
        // >
        // > The expression attribute value :v_sub is a placeholder for http://.
        //
        // Source: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions
        S: Into<StringOrRef>,
    {
        Self {
            path: path.into(),
            substr: substr.into().into(),
        }
    }
}

impl fmt::Display for BeginsWith {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("begins_with(")?;
        self.path.fmt(f)?;
        f.write_str(", ")?;
        self.substr.fmt(f)?;
        f.write_char(')')
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use pretty_assertions::assert_eq;

    use crate::{value::Ref, Path};

    use super::BeginsWith;

    #[test]
    fn string() -> Result<(), Box<dyn Error>> {
        let begins_with = BeginsWith::new("foo[3]".parse::<Path>()?, "foo");
        assert_eq!(r#"begins_with(foo[3], "foo")"#, begins_with.to_string());

        let begins_with = BeginsWith::new("foo[3]".parse::<Path>()?, String::from("foo"));
        assert_eq!(r#"begins_with(foo[3], "foo")"#, begins_with.to_string());

        #[expect(
            clippy::needless_borrows_for_generic_args,
            reason = "Explicitly testing &String"
        )]
        let begins_with = BeginsWith::new("foo[3]".parse::<Path>()?, &String::from("foo"));
        assert_eq!(r#"begins_with(foo[3], "foo")"#, begins_with.to_string());

        #[expect(
            clippy::needless_borrows_for_generic_args,
            reason = "Explicitly testing &&str"
        )]
        let begins_with = BeginsWith::new("foo[3]".parse::<Path>()?, &"foo");
        assert_eq!(r#"begins_with(foo[3], "foo")"#, begins_with.to_string());

        Ok(())
    }

    #[test]
    fn value_ref() {
        let begins_with = BeginsWith::new("foo[3]".parse::<Path>().unwrap(), Ref::from("prefix"));
        assert_eq!("begins_with(foo[3], :prefix)", begins_with.to_string());
    }
}
