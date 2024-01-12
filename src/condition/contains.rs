use core::fmt::{self, Write};

use crate::{
    path::Path,
    value::{Value, ValueOrRef},
};

/// The [DynamoDB `contains` function][1]. True if the attribute specified
/// by [`Path`] is one of the following:
/// * A `String` that contains a particular substring.
/// * A `Set` that contains a particular element within the set.
/// * A `List` that contains a particular element within the list.
///
/// The operand must be a `String` if the attribute specified by path is a
/// `String`. If the attribute specified by path is a `Set`, the operand
/// must be the sets element type.
///
/// See also: [`Path::contains`]
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use dynamodb_expression::{Num, Path};
///
/// // String
/// let condition = "foo".parse::<Path>()?.contains("Quinn");
/// assert_eq!(r#"contains(foo, "Quinn")"#, condition.to_string());
///
/// // Number
/// let condition = "foo".parse::<Path>()?.contains(Num::new(42));
/// assert_eq!(r#"contains(foo, 42)"#, condition.to_string());
///
/// // Binary
/// let condition = "foo".parse::<Path>()?.contains(Vec::<u8>::from("fish"));
/// assert_eq!(r#"contains(foo, "ZmlzaA==")"#, condition.to_string());
/// #
/// # Ok(())
/// # }
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contains {
    // `Path` is correct here
    // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Syntax
    pub(crate) path: Path,
    pub(crate) operand: ValueOrRef,
}

impl Contains {
    /// Allows for manually creating a `Contains` instance.
    ///
    /// See also: [`Path::contains`]
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use dynamodb_expression::{condition::Contains, Num, Path};
    ///
    /// // String
    /// let condition =  Contains::new("foo".parse::<Path>()?, "Quinn");
    /// assert_eq!(r#"contains(foo, "Quinn")"#, condition.to_string());
    ///
    /// // Number
    /// let condition = Contains::new("foo".parse::<Path>()?, Num::new(42));
    /// assert_eq!(r#"contains(foo, 42)"#, condition.to_string());
    ///
    /// // Binary
    /// let condition = Contains::new("foo".parse::<Path>()?, Vec::<u8>::from("fish"));
    /// assert_eq!(r#"contains(foo, "ZmlzaA==")"#, condition.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<P, S>(path: P, operand: S) -> Self
    where
        P: Into<Path>,
        S: Into<Value>,
    {
        Self {
            path: path.into(),
            operand: operand.into().into(),
        }
    }
}

impl fmt::Display for Contains {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("contains(")?;
        self.path.fmt(f)?;
        f.write_str(", ")?;
        self.operand.fmt(f)?;
        f.write_char(')')
    }
}
