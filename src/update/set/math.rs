use core::fmt::{self, Write};

use crate::{
    path::Path,
    update::{set_remove::SetRemove, Set},
    value::{Num, ValueOrRef},
};

/// Represents a [DynamoDB math operation][1] used as a part of an update expression.
///
/// See also: [`Path::math`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.IncrementAndDecrement
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Math {
    pub(crate) dst: Path,
    pub(crate) src: Option<Path>,
    op: MathOp,
    pub(crate) num: ValueOrRef,
}

/// A [math operation][1] to modify a field and assign the updated value
/// to another (possibly different) field.
///
/// See also: [`Path::math`]
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html#Expressions.UpdateExpressions.SET.IncrementAndDecrement
impl Math {
    pub fn builder<T>(dst: T) -> Builder
    where
        T: Into<Path>,
    {
        Builder {
            dst: dst.into(),
            src: None,
        }
    }

    /// Add an additional [`Set`] or [`Remove`] statement to this expression.
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// use dynamodb_expression::{Num, Path};
    /// # use pretty_assertions::assert_eq;
    ///
    /// let set = "foo"
    ///     .parse::<Path>()?
    ///     .math()
    ///     .add(1)
    ///     .and("bar".parse::<Path>()?.set("a value"));
    /// assert_eq!(r#"SET foo = foo + 1, bar = "a value""#, set.to_string());
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Remove`]: crate::update::Remove
    pub fn and<T>(self, other: T) -> SetRemove
    where
        T: Into<SetRemove>,
    {
        Set::from(self).and(other)
    }
}

impl fmt::Display for Math {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.dst.fmt(f)?;
        f.write_str(" = ")?;
        // If no source field is specified, default to using the destination field.
        self.src.as_ref().unwrap_or(&self.dst).fmt(f)?;
        f.write_char(' ')?;
        self.op.fmt(f)?;
        f.write_char(' ')?;
        self.num.fmt(f)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MathOp {
    Add,
    Sub,
}

impl fmt::Debug for MathOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for MathOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            MathOp::Add => '+',
            MathOp::Sub => '-',
        })
    }
}

/// See: [`Path::math`]
#[must_use = "Consume this `Builder` by using its `.add()` or `.sub()` methods"]
#[derive(Debug, Clone)]
pub struct Builder {
    dst: Path,
    src: Option<Path>,
}

impl Builder {
    /// Sets the source field to read the initial value from.
    /// Defaults to the destination field.
    pub fn src<T>(mut self, src: T) -> Self
    where
        T: Into<Path>,
    {
        self.src = Some(src.into());

        self
    }

    /// Sets addition as the operation to perform.
    #[allow(clippy::should_implement_trait)]
    pub fn add<T>(self, num: T) -> Math
    where
        T: Into<Num>,
    {
        self.with_op(MathOp::Add, num)
    }

    /// Sets subtraction as the operation to perform.
    #[allow(clippy::should_implement_trait)]
    pub fn sub<T>(self, num: T) -> Math
    where
        T: Into<Num>,
    {
        self.with_op(MathOp::Sub, num)
    }

    fn with_op<T>(self, op: MathOp, num: T) -> Math
    where
        T: Into<Num>,
    {
        let Self { dst, src } = self;

        Math {
            dst,
            src,
            op,
            num: num.into().into(),
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::{
        update::{set_remove::SetRemove, Assign, Set, SetAction},
        Num, Path,
    };

    use super::Math;

    #[test]
    fn and() -> Result<(), Box<dyn std::error::Error>> {
        let math: Math = "foo".parse::<Path>()?.math().add(1);
        let assign: Assign = "bar".parse::<Path>()?.set(Num::new(8));

        // Should be able to concatenate anything that can be turned into a SetAction.

        let combined = math.clone().and(assign.clone());
        assert_eq!(r#"SET foo = foo + 1, bar = 8"#, combined.to_string());

        // Should be able to concatenate a SetAction instance.

        let combined = math.clone().and(SetAction::from(assign.clone()));
        assert_eq!(r#"SET foo = foo + 1, bar = 8"#, combined.to_string());

        // Should be able to concatenate a Set instance

        let set: Set = [
            SetAction::from(assign),
            SetAction::from("baz".parse::<Path>()?.list_append().list(["d", "e", "f"])),
        ]
        .into_iter()
        .collect();
        let combined = math.clone().and(set);
        assert_eq!(
            r#"SET foo = foo + 1, bar = 8, baz = list_append(baz, ["d", "e", "f"])"#,
            combined.to_string()
        );

        // Should be able to concatenate a Remove instance

        let combined = math.clone().and("quux".parse::<Path>()?.remove());
        assert_eq!(r#"SET foo = foo + 1 REMOVE quux"#, combined.to_string());

        // Should be able to concatenate a SetRemove instance

        let combined = math.and(SetRemove::from("quux".parse::<Path>()?.remove()));
        assert_eq!(r#"SET foo = foo + 1 REMOVE quux"#, combined.to_string());

        Ok(())
    }
}
