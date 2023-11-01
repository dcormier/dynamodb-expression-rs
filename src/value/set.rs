use core::fmt;
use std::collections::BTreeSet;

use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue};

use super::base64;

/// A collection of DynamoDB values that are all the same type and unique.
///
/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Set {
    StringSet(StringSet),
    NumSet(NumSet),
    BinarySet(BinarySet),
}

impl Set {
    // Intentionally not using `impl From<SetValue> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    pub(super) fn into_attribute_value(self) -> AttributeValue {
        match self {
            Set::StringSet(set) => set.into_attribute_value(),
            Set::NumSet(set) => set.into_attribute_value(),
            Set::BinarySet(set) => set.into_attribute_value(),
        }
    }
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Set::StringSet(set) => set.fmt(f),
            Set::NumSet(set) => set.fmt(f),
            Set::BinarySet(set) => set.fmt(f),
        }
    }
}

/// A set of unique string values for DynamoDB
///
/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringSet(BTreeSet<String>);

impl StringSet {
    // Intentionally not using `impl From<StringSet> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::Ss(self.0.into_iter().collect())
    }
}

impl<T> FromIterator<T> for StringSet
where
    T: Into<String>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

impl<I, T> From<I> for StringSet
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    fn from(values: I) -> Self {
        Self::from_iter(values)
    }
}

impl fmt::Display for StringSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

/// A set of unique string values for DynamoDB
///
/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes>
pub fn string_set<I, T>(set: I) -> Set
where
    I: IntoIterator<Item = T>,
    T: Into<String>,
{
    Set::StringSet(set.into())
}

/// A set of unique numeric values for DynamoDB
///
/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumSet(BTreeSet<String>);

impl NumSet {
    pub fn insert<T>(&mut self, num: T)
    where
        T: ToString + num::Num,
    {
        self.0.insert(Self::into_num(num));
    }

    /// Converts a numeric type into a DynamoDB numeric value
    fn into_num<T>(num: T) -> String
    where
        T: ToString + num::Num,
    {
        num.to_string()
    }

    // Intentionally not using `impl From<NumSet> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::Ns(self.0.into_iter().collect())
    }
}

impl<T> FromIterator<T> for NumSet
where
    T: ToString + num::Num,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(iter.into_iter().map(Self::into_num).collect())
    }
}

impl<I, T> From<I> for NumSet
where
    I: IntoIterator<Item = T>,
    T: ToString + num::Num,
{
    fn from(values: I) -> Self {
        Self::from_iter(values)
    }
}

impl fmt::Display for NumSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

/// A set of unique numeric values for DynamoDB
///
/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes>
pub fn num_set<I, T>(set: I) -> Set
where
    I: IntoIterator<Item = T>,
    T: ToString + num::Num,
{
    Set::NumSet(set.into())
}

/// A set of unique binary values for DynamoDB
///
/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinarySet(BTreeSet<Vec<u8>>);

impl BinarySet {
    // Intentionally not using `impl From<BinarySet> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::Bs(self.0.into_iter().map(Blob::new).collect())
    }
}

impl<T> FromIterator<T> for BinarySet
where
    T: IntoIterator<Item = u8>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self(
            iter.into_iter()
                .map(|value| value.into_iter().collect())
                .collect(),
        )
    }
}

impl<I, T> From<I> for BinarySet
where
    I: IntoIterator<Item = T>,
    T: IntoIterator<Item = u8>,
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

/// A set of unique binary values for DynamoDB
///
/// <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes>
pub fn binary_set<I, T>(set: I) -> Set
where
    I: IntoIterator<Item = T>,
    T: IntoIterator<Item = u8>,
{
    Set::BinarySet(set.into())
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, iter::FusedIterator};

    use itertools::Itertools;
    use pretty_assertions::assert_eq;

    use crate::{binary_set, num_set, string_set, value::base64};

    #[test]
    fn string_set_display() {
        let set = string_set(["foo", "bar", "!@#$%^&*()-=_+\"'{}[]\\|;:<>,./?`~"]);
        assert_eq!(
            r#"["!@#$%^&*()-=_+\"'{}[]\\|;:<>,./?`~", "bar", "foo"]"#,
            set.to_string()
        );

        let deserialized: Vec<String> =
            serde_json::from_str(&set.to_string()).expect("Must be valid JSON");
        assert_eq!(
            vec!["!@#$%^&*()-=_+\"'{}[]\\|;:<>,./?`~", "bar", "foo"],
            deserialized
        );
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn num_set_display() {
        let set = num_set([-1, 0, 1, 42]);
        assert_eq!(r#"["-1", "0", "1", "42"]"#, set.to_string());

        let deserialized: Vec<String> =
            serde_json::from_str(&set.to_string()).expect("Must be valid JSON");
        assert_eq!(vec!["-1", "0", "1", "42"], deserialized);

        let set = num_set([f64::MIN, 0.0, 3.14, f64::MAX]);
        assert_eq!(
            "[\"-17976931348623157000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000\", \
            \"0\", \
            \"17976931348623157000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000000000000000000000000000\
            0000000000000000000000000000000000000000000\", \
            \"3.14\"]",
            set.to_string()
        );

        let deserialized: Vec<String> =
            serde_json::from_str(&set.to_string()).expect("Must be valid JSON");
        assert_eq!(
            vec![
                "-17976931348623157000000000000000000000000000000000000000000000\
                    0000000000000000000000000000000000000000000000000000000000000000000\
                    0000000000000000000000000000000000000000000000000000000000000000000\
                    0000000000000000000000000000000000000000000000000000000000000000000\
                    0000000000000000000000000000000000000000000000",
                "0",
                "17976931348623157000000000000000000000000000000000000000000000000\
                    0000000000000000000000000000000000000000000000000000000000000000000\
                    0000000000000000000000000000000000000000000000000000000000000000000\
                    0000000000000000000000000000000000000000000000000000000000000000000\
                    0000000000000000000000000000000000000000000",
                "3.14",
            ],
            deserialized
        );
    }

    #[test]
    fn binary_set_display() {
        // These strings chosen because they produce base64 strings with all the
        // non-alphanumeric chars in the base64 set ('+', '/', and the padding
        // char, '='). Used `find_tricky_base64()`, below.
        let set = binary_set(["  > ", "  ? "].into_iter().map(str::bytes));
        assert_eq!(r#"["ICA+IA==", "ICA/IA=="]"#, set.to_string());

        let deserialized: Vec<String> =
            serde_json::from_str(&set.to_string()).expect("Must be valid JSON");
        assert_eq!(vec!["ICA+IA==", "ICA/IA=="], deserialized);
    }

    #[test]
    #[ignore = "Just used to find more base64 for JSON encoding testing"]
    fn find_tricky_base64() {
        /// Visible ASCII characters
        fn charset(
        ) -> impl Iterator<Item = char> + ExactSizeIterator + DoubleEndedIterator + FusedIterator + Clone
        {
            (32..127).map(char::from_u32).map(Option::unwrap)
        }

        // Check that the encoded value contains at least one of the
        // non-alphanumeric (and non-padding) base64 chars.
        let specials = RefCell::new(['+', '/'].into_iter().peekable());
        let values = [charset(), charset(), charset(), charset()]
            .into_iter()
            .multi_cartesian_product()
            .take_while(|_| specials.borrow_mut().peek().is_some())
            .map(String::from_iter)
            .enumerate() // Just to see how many iterations this takes
            .map(|(i, raw)| {
                let encoded = base64(&raw);
                (i, raw, encoded)
            })
            .filter(|(_i, _raw, encoded)| {
                if encoded.contains(specials.borrow_mut().peek().cloned().unwrap()) {
                    specials.borrow_mut().next();
                    true
                } else {
                    false
                }
            })
            .collect_vec();

        for (index, raw, encoded) in values {
            println!(
                "The encoded version of iteration {index}, {raw:?}, \
                        includes special characters: {encoded}"
            )
        }
    }
}
