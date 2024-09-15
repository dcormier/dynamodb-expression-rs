use core::fmt;
use std::collections::BTreeSet;

use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue};

use super::base64;

/// Represents a [DynamoDB binary set][1].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinarySet(BTreeSet<Vec<u8>>);

impl BinarySet {
    /// Creates a value to use as a [DynamoDB binary set][1].
    ///
    /// It can be created from `IntoIterator<T>` where `T` is `Into<Vec<u8>>`.
    ///
    /// See also: [`Set::new_binary_set`], [`Value::new_binary_set`]
    ///
    /// # Examples
    /// ```
    /// use dynamodb_expression::value::BinarySet;
    /// # use pretty_assertions::assert_eq;
    ///
    /// // impl IntoIterator<Item = &str>
    /// assert_eq!(
    ///     r#"["YQ==", "Yg==", "Yw=="]"#,
    ///     BinarySet::new(["a", "b", "c"]).to_string()
    /// );
    ///
    /// // impl IntoIterator<Item = String>
    /// assert_eq!(
    ///     r#"["YQ==", "Yg==", "Yw=="]"#,
    ///     BinarySet::new([String::from("a"), String::from("b"), String::from("c")]).to_string()
    /// );
    ///
    /// // impl IntoIterator<Item = &[u8]>
    /// assert_eq!(
    ///     r#"["YQ==", "Yg==", "Yw=="]"#,
    ///     BinarySet::new([b"a", b"b", b"c"]).to_string()
    /// );
    ///
    /// // impl IntoIterator<Item = Vec<u8>>
    /// assert_eq!(
    ///     r#"["YQ==", "Yg==", "Yw=="]"#,
    ///     BinarySet::new([b"a".to_vec(), b"b".to_vec(), b"c".to_vec()]).to_string()
    /// );
    ///
    /// // impl IntoIterator<Item = impl Iterator<Item = u8>>
    /// assert_eq!(
    ///     r#"["YQ==", "Yg==", "Yw=="]"#,
    ///     BinarySet::new(
    ///         [[b'a'], [b'b'], [b'c']]
    ///             // In this case you need to turn the `Iterator<Item = u8>` items into a `Vec<u8>`.
    ///             // If, one day, `impl<T> From<impl IntoIterator<Item = T>> for Vec<T>` exists, this won't be needed.
    ///             .map(Vec::from_iter)
    ///     )
    ///     .to_string()
    /// );
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
    /// [`Set::new_binary_set`]: crate::Set::new_binary_set
    /// [`Value::new_binary_set`]: crate::Value::new_binary_set
    pub fn new<T>(set: T) -> Self
    where
        T: Into<BinarySet>,
    {
        set.into()
    }

    // Intentionally not using `impl From<BinarySet> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    pub(super) fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::Bs(self.0.into_iter().map(Blob::new).collect())
    }
}

impl<T> FromIterator<T> for BinarySet
where
    T: Into<Vec<u8>>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

impl<I, T> From<I> for BinarySet
where
    I: IntoIterator<Item = T>,
    T: Into<Vec<u8>>,
{
    fn from(values: I) -> Self {
        Self::from_iter(values)
    }
}

impl fmt::Display for BinarySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter().map(base64)).finish()
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::BinarySet;
    use crate::Scalar;

    #[test]
    fn from_iter() {
        // impl IntoIterator<Item = &str>
        assert_eq!(
            r#"["YQ==", "Yg==", "Yw=="]"#,
            BinarySet::new(["a", "b", "c"]).to_string()
        );

        // impl IntoIterator<Item = String>
        assert_eq!(
            r#"["YQ==", "Yg==", "Yw=="]"#,
            BinarySet::new([String::from("a"), String::from("b"), String::from("c")]).to_string()
        );

        // impl IntoIterator<Item = &[u8]>
        assert_eq!(
            r#"["YQ==", "Yg==", "Yw=="]"#,
            BinarySet::new([b"a", b"b", b"c"]).to_string()
        );

        // impl IntoIterator<Item = Vec<u8>>
        assert_eq!(
            r#"["YQ==", "Yg==", "Yw=="]"#,
            BinarySet::new([b"a".to_vec(), b"b".to_vec(), b"c".to_vec()]).to_string()
        );

        // impl IntoIterator<Item = impl Iterator<Item = u8>>
        assert_eq!(
            r#"["YQ==", "Yg==", "Yw=="]"#,
            BinarySet::new(
                [[b'a'], [b'b'], [b'c']]
                    // In this case you need to turn the `Iterator<Item = u8>` items into a `Vec<u8>`.
                    // If, one day, `impl<T> From<impl IntoIterator<Item = T>> for Vec<T>` exists, this won't be needed.
                    .map(Vec::from_iter)
            )
            .to_string()
        );
    }

    #[test]
    fn comparable_with_binary() {
        // &str
        assert_eq!(r#""YQ==""#, Scalar::new_binary("a").to_string());
        assert_eq!(
            r#"["YQ==", "Yg==", "Yw=="]"#,
            BinarySet::new(["a", "b", "c"]).to_string()
        );

        // String
        assert_eq!(
            r#""YQ==""#,
            Scalar::new_binary(String::from("a")).to_string()
        );
        assert_eq!(
            r#"["YQ==", "Yg==", "Yw=="]"#,
            BinarySet::new([String::from("a"), String::from("b"), String::from("c")]).to_string()
        );

        // &[u8]
        assert_eq!(r#""YQ==""#, Scalar::new_binary(b"a").to_string());
        assert_eq!(
            r#"["YQ==", "Yg==", "Yw=="]"#,
            BinarySet::new([b"a", b"b", b"c"]).to_string()
        );

        // Vec<u8>
        assert_eq!(r#""YQ==""#, Scalar::new_binary(b"a".to_vec()).to_string());
        assert_eq!(
            r#"["YQ==", "Yg==", "Yw=="]"#,
            BinarySet::new([b"a".to_vec(), b"b".to_vec(), b"c".to_vec()]).to_string()
        );

        // impl Iterator<Item = u8>
        assert_eq!(
            r#""YQ==""#,
            Scalar::new_binary(Vec::from_iter("a".bytes())).to_string()
        );
        assert_eq!(
            r#"["YQ==", "Yg==", "Yw=="]"#,
            BinarySet::new(["a".bytes(), "b".bytes(), "c".bytes()].map(Vec::from_iter)).to_string()
        );
    }
}
