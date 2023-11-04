use core::fmt::{self, LowerExp, UpperExp};

use aws_sdk_dynamodb::types::AttributeValue;

/// A DynamoDB [numeric][1] value.
///
/// See also: [`Scalar::new_num`], [`Value::new_num`],
/// [`Scalar::new_num_lower_exp`], [`Value::new_num_lower_exp`],
/// [`Scalar::new_num_upper_exp`], [`Value::new_num_upper_exp`]
///
/// # Examples
///
/// ```
/// use dynamodb_expression::value::Num;
/// # use pretty_assertions::assert_eq;
///
/// let value = Num::new(2600);
/// assert_eq!("2600", value.to_string());
///
/// let value = Num::new_lower_exp(2600);
/// assert_eq!("2.6e3", value.to_string());
///
/// let value = Num::new_upper_exp(2600);
/// assert_eq!("2.6E3", value.to_string());
///
/// let value = Num::new(2600.0);
/// assert_eq!("2600", value.to_string());
///
/// let value = Num::new_lower_exp(2600.0);
/// assert_eq!("2.6e3", value.to_string());
///
/// let value = Num::new_upper_exp(2600.0);
/// assert_eq!("2.6E3", value.to_string());
/// ```
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.Number
/// [`Scalar::new_num`]: crate::value::Scalar::new_num
/// [`Scalar::new_num_lower_exp`]: crate::value::Scalar::new_num_lower_exp
/// [`Scalar::new_num_upper_exp`]: crate::value::Scalar::new_num_upper_exp
/// [`Value::new_num`]: crate::value::Value::new_num
/// [`Value::new_num_lower_exp`]: crate::value::Value::new_num_lower_exp
/// [`Value::new_num_upper_exp`]: crate::value::Value::new_num_upper_exp
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Num {
    n: String,
}

impl Num {
    /// Creates a DynamoDB [numeric][1] value.
    ///
    /// See also: [`Num::new_lower_exp`], [`Num::new_upper_exp`], [`Scalar::new_num`],
    /// [`Value::new_num`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::value::Num;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let value = Num::new(2600);
    /// assert_eq!("2600", value.to_string());
    ///
    /// let value = Num::new(2600.0);
    /// assert_eq!("2600", value.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.Number
    /// [`Scalar::new_num`]: crate::value::Scalar::new_num
    /// [`Value::new_num`]: crate::value::Value::new_num
    pub fn new<T>(value: T) -> Self
    where
        T: ToString + num::Num,
    {
        Self {
            n: value.to_string(),
        }
    }

    /// Creates a DynamoDB [numeric][1] value.
    ///
    /// See also: [`Num::new`], [`Num::new_upper_exp`], [`Scalar::new_num_lower_exp`],
    /// [`Value::new_num_lower_exp`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::value::Num;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let value = Num::new_lower_exp(2600);
    /// assert_eq!("2.6e3", value.to_string());
    ///
    /// let value = Num::new_lower_exp(2600.0);
    /// assert_eq!("2.6e3", value.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.Number
    /// [`Scalar::new_num_lower_exp`]: crate::value::Scalar::new_num_lower_exp
    /// [`Value::new_num_lower_exp`]: crate::value::Value::new_num_lower_exp
    pub fn new_lower_exp<T>(value: T) -> Self
    where
        T: LowerExp + num::Num,
    {
        Self {
            n: format!("{value:e}"),
        }
    }

    /// Creates a DynamoDB [numeric][1] value.
    ///
    /// See also: [`Num::new`], [`Num::new_lower_exp`], [`Scalar::new_num_upper_exp`],
    /// [`Value::new_num_upper_exp`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dynamodb_expression::value::Num;
    /// # use pretty_assertions::assert_eq;
    ///
    /// let value = Num::new_upper_exp(2600);
    /// assert_eq!("2.6E3", value.to_string());
    ///
    /// let value = Num::new_upper_exp(2600.0);
    /// assert_eq!("2.6E3", value.to_string());
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes.Number
    /// [`Scalar::new_num_upper_exp`]: crate::value::Scalar::new_num_upper_exp
    /// [`Value::new_num_upper_exp`]: crate::value::Value::new_num_upper_exp
    pub fn new_upper_exp<T>(value: T) -> Self
    where
        T: UpperExp + num::Num,
    {
        Self {
            n: format!("{value:E}"),
        }
    }

    // Intentionally not using `impl From<Num> for AttributeValue` because
    // I don't want to make this a public API people rely on. The purpose of this
    // crate is not to make creating `AttributeValues` easier. They should try
    // `serde_dynamo`.
    pub(super) fn into_attribute_value(self) -> AttributeValue {
        AttributeValue::N(self.n)
    }
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.n.fmt(f)
    }
}

impl<T> From<T> for Num
where
    T: ToString + num::Num,
{
    fn from(num: T) -> Self {
        Num::new(num)
    }
}

impl From<Num> for String {
    fn from(num: Num) -> Self {
        num.n
    }
}
