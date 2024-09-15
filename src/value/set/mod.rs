mod binary_set;
mod num_set;
mod string_set;

pub use binary_set::BinarySet;
pub use num_set::NumSet;
pub use string_set::StringSet;

use core::fmt;

use aws_sdk_dynamodb::types::AttributeValue;

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
    /// Creates a value to use as a [DynamoDB string set][1].
    ///
    /// See also: [`StringSet::new`], [`Value::new_string_set`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
    /// [`Value::new_string_set`]: crate::Value::new_string_set
    pub fn new_string_set<T>(string_set: T) -> Self
    where
        T: Into<StringSet>,
    {
        string_set.into().into()
    }

    /// Creates a value to use as a [DynamoDB number set][1].
    ///
    /// See also: [`NumSet::new`], [`Value::new_num_set`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
    /// [`Value::new_num_set`]: crate::Value::new_num_set
    pub fn new_num_set<T>(num_set: T) -> Self
    where
        T: Into<NumSet>,
    {
        num_set.into().into()
    }

    /// Creates a value to use as a [DynamoDB binary set][1].
    ///
    /// See also: [`BinarySet::new`], [`Value::new_binary_set`]
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.SetTypes
    /// [`Value::new_binary_set`]: crate::Value::new_binary_set
    pub fn new_binary_set<T>(binary_set: T) -> Self
    where
        T: Into<BinarySet>,
    {
        binary_set.into().into()
    }

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

impl From<StringSet> for Set {
    fn from(string_set: StringSet) -> Self {
        Self::StringSet(string_set)
    }
}

impl From<NumSet> for Set {
    fn from(num_set: NumSet) -> Self {
        Self::NumSet(num_set)
    }
}

impl From<BinarySet> for Set {
    fn from(binary_set: BinarySet) -> Self {
        Self::BinarySet(binary_set)
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, iter::FusedIterator};

    use itertools::Itertools;
    use pretty_assertions::assert_eq;

    use crate::{
        value::{base64, Set},
        Num,
    };

    #[test]
    fn string_set_display() {
        let set = Set::new_string_set(["foo", "bar", "!@#$%^&*()-=_+\"'{}[]\\|;:<>,./?`~"]);
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
    fn num_set_display() {
        let set = Set::new_num_set([-1, 0, 1, 42]);
        assert_eq!("[-1, 0, 1, 42]", set.to_string());

        let deserialized: Vec<i32> =
            serde_json::from_str(&set.to_string()).expect("Must be valid JSON");
        assert_eq!(vec![-1, 0, 1, 42], deserialized);

        let set = Set::new_num_set([
            Num::new_lower_exp(f32::MIN),
            Num::new(0.0),
            Num::new(1000),
            Num::new_upper_exp(f32::MAX),
            Num::new(9.2),
        ]);
        assert_eq!(
            "[\
                -3.4028235e38, \
                0, \
                1000, \
                3.4028235E38, \
                9.2\
            ]",
            set.to_string()
        );

        let deserialized: Vec<f32> =
            serde_json::from_str(&set.to_string()).expect("Must be valid JSON");
        assert_eq!(vec![f32::MIN, 0.0, 1000.0, f32::MAX, 9.2], deserialized);
    }

    #[test]
    fn binary_set_display() {
        // These strings chosen because they produce base64 strings with all the
        // non-alphanumeric chars in the base64 set ('+', '/', and the padding
        // char, '='). Used `find_tricky_base64()`, below.
        let set = Set::new_binary_set(["  > ", "  ? "]);
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
        ) -> impl ExactSizeIterator<Item = char> + DoubleEndedIterator + FusedIterator + Clone
        {
            (32..127).map(char::from_u32).map(Option::unwrap)
        }

        // Check that the encoded value contains at least one of the
        // non-alphanumeric (and non-padding) base64 chars.
        let specials = RefCell::new(['+', '/'].into_iter().peekable());
        [charset(), charset(), charset(), charset()]
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
            .for_each(|(index, raw, encoded)| {
                println!(
                    "The encoded version of iteration {index}, {raw:?}, \
                        includes special characters: {encoded}"
                )
            });
    }
}
