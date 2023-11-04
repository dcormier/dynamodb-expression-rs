use core::fmt;

use crate::{
    path::Path,
    value::{StringOrRef, ValueOrRef},
};

/// True if the attribute specified by `path` begins with a particular substring.
///
/// [DynamoDB documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Functions)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeginsWith {
    // `Path` is correct here
    // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Syntax
    pub(crate) path: Path,
    pub(crate) substr: ValueOrRef,
}

impl BeginsWith {
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
        write!(f, "begins_with({}, {})", self.path, self.substr)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::{value::Ref, Path};

    use super::BeginsWith;

    #[test]
    fn string() {
        let begins_with = BeginsWith::new(Path::new_indexed_field("foo", 3), "foo");
        assert_eq!(r#"begins_with(foo[3], "foo")"#, begins_with.to_string());

        let begins_with = BeginsWith::new(Path::new_indexed_field("foo", 3), String::from("foo"));
        assert_eq!(r#"begins_with(foo[3], "foo")"#, begins_with.to_string());

        #[allow(clippy::needless_borrow)]
        let begins_with = BeginsWith::new(Path::new_indexed_field("foo", 3), &String::from("foo"));
        assert_eq!(r#"begins_with(foo[3], "foo")"#, begins_with.to_string());

        #[allow(clippy::needless_borrow)]
        let begins_with = BeginsWith::new(Path::new_indexed_field("foo", 3), &"foo");
        assert_eq!(r#"begins_with(foo[3], "foo")"#, begins_with.to_string());
    }

    #[test]
    fn value_ref() {
        let begins_with = BeginsWith::new(Path::new_indexed_field("foo", 3), Ref::from("prefix"));
        assert_eq!("begins_with(foo[3], :prefix)", begins_with.to_string());
    }
}
