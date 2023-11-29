mod builder;
mod to_aws;

pub use builder::Builder;

use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;

/// The data needed for various [`aws_sdk_dynamodb`] input types.
///
/// Use [`Builder`] (via [`Expression::builder`]) to create a DynamoDB
/// condition, filter, key condition, or update expression.
///
/// You can use the these methods (see their docs for examples) to consume this
/// and set up a DynamoDB operation:
/// * [`Expression::put_item`]
/// * [`Expression::get_item`]
/// * [`Expression::scan`]
/// * [`Expression::query`]
/// * [`Expression::update_item`]
/// * [`Expression::delete_item`]
///
/// See also the `to_*_input_builder` and `to_*_fluent_builder` methods for
/// populating [`aws_sdk_dynamodb`] types.
///
/// You can use the fields on this struct to manually populate an
/// [`aws_sdk_dynamodb`] input type, or you can use one of the many methods on
/// this to automatically build or populate one of those types.
///
#[must_use = "Use the fields or methods to create an input type for `aws_sdk_dynamodb"]
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    /// The string to use as a DynamoDB [condition expression][1].
    ///
    /// Be sure to also use [`.expression_attribute_names`] and
    /// [`.expression_attribute_values`].
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ConditionExpressions.html
    /// [`.expression_attribute_names`]: Self::expression_attribute_names
    /// [`.expression_attribute_values`]: Self::expression_attribute_values
    pub condition_expression: Option<String>,

    /// The string to use use as a DynamoDB [key condition expression][1].
    ///
    /// Be sure to also use [`.expression_attribute_names`] and
    /// [`.expression_attribute_values`].
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Query.KeyConditionExpressions.html
    /// [`.expression_attribute_names`]: Self::expression_attribute_names
    /// [`.expression_attribute_values`]: Self::expression_attribute_values
    pub key_condition_expression: Option<String>,

    /// The string to use as a DynamoDB [update expression][1].
    ///
    /// Be sure to also use [`.expression_attribute_names`] and
    /// [`.expression_attribute_values`].
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html
    /// [`.expression_attribute_names`]: Self::expression_attribute_names
    /// [`.expression_attribute_values`]: Self::expression_attribute_values
    pub update_expression: Option<String>,

    /// The string to use as a DynamoDB [filter expression][1].
    ///
    /// Be sure to also use [`.expression_attribute_names`] and
    /// [`.expression_attribute_values`].
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Query.FilterExpression.html
    /// [`.expression_attribute_names`]: Self::expression_attribute_names
    /// [`.expression_attribute_values`]: Self::expression_attribute_values
    pub filter_expression: Option<String>,

    /// The string to use as a DynamoDB [projection expression][1].
    ///
    /// Be sure to also use [`.expression_attribute_names`].
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ProjectionExpressions.html
    /// [`.expression_attribute_names`]: Self::expression_attribute_names
    pub projection_expression: Option<String>,

    /// DynamoDB [expression attribute names][1].
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeNames.html
    pub expression_attribute_names: Option<HashMap<String, String>>,

    /// DynamoDB [expression attribute values][1].
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeValues.html
    pub expression_attribute_values: Option<HashMap<String, AttributeValue>>,
}

impl Expression {
    pub fn builder() -> Builder {
        Builder::default()
    }
}
