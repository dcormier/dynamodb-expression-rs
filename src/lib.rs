/*!
A crate to help build DynamoDB condition, filter, key condition, and update
expressions in a type-safe way, including handling [expression attribute names][1]
and [expression attribute values][2].

[`Expression`] is the type to use for a [DynamoDB expression][2].
[`Path`] represents a [DynamoDB item attribute or document path][3], and has
many methods for building various expressions.

An example showing a how to use this crate to perform a query:

```no_run
# async fn example_query() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use dynamodb_expression::{Expression, Num, Path};

let client = Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await);

let query_output = Expression::builder()
    .with_filter(
        "name"
            .parse::<Path>()?
            .attribute_exists()
            .and("age".parse::<Path>()?.greater_than_or_equal(Num::new(2.5))),
    )
    .with_projection(["name", "age"])
    .with_key_condition("id".parse::<Path>()?.key().equal(Num::new(42)))
    .build()
    .query(&client)
    .table_name("people")
    .send()
    .await?;
#
# _ = query_output;
# Ok(())
# }
```

From here, see [`Expression`] and [`Path`] for more docs and examples.

# What about Rusoto?

[Rusoto][5] is intentionally not supported.

If you are using Rusoto and want to take advantage of this crate, you can still
build an [`Expression`], then convert the [`aws_sdk_dynamodb::types::AttributeValue`]
that are in the `expression_attribute_values` field into [`rusoto_dynamodb::AttributeValue`].
The rest of the fields are already what's needed.

```no_run
# async fn example_rusoto() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue as AwsAv};
use dynamodb_expression::{Expression, Num, Path};
use itermap::IterMap;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue as RusotoAv, DynamoDb, DynamoDbClient, QueryInput};

let expression = Expression::builder()
    .with_filter(
        "name"
            .parse::<Path>()?
            .attribute_exists()
            .and("age".parse::<Path>()?.greater_than_or_equal(Num::new(2.5))),
    )
    .with_projection(["name", "age"])
    .with_key_condition("id".parse::<Path>()?.key().equal(Num::new(42)))
    .build();

let input = QueryInput {
    filter_expression: expression.filter_expression,
    projection_expression: expression.projection_expression,
    key_condition_expression: expression.key_condition_expression,
    expression_attribute_names: expression.expression_attribute_names,
    expression_attribute_values: expression
        .expression_attribute_values
        .map(|values| values.into_iter().map_values(convert_av).collect()),
    table_name: String::from("people"),
    ..QueryInput::default()
};

let output = DynamoDbClient::new(Region::UsEast1).query(input).await?;

fn convert_av(av: AwsAv) -> RusotoAv {
    let mut rav = RusotoAv::default();

    match av {
        AwsAv::B(av) => rav.b = Some(av.into_inner().into()),
        AwsAv::Bool(av) => rav.bool = av.into(),
        AwsAv::Bs(av) => {
            rav.bs = Some(
                av.into_iter()
                    .map(Blob::into_inner)
                    .map(Into::into)
                    .collect(),
            )
        }
        AwsAv::L(av) => rav.l = Some(av.into_iter().map(convert_av).collect()),
        AwsAv::M(av) => rav.m = Some(av.into_iter().map_values(convert_av).collect()),
        AwsAv::N(av) => rav.n = av.into(),
        AwsAv::Ns(av) => rav.ns = av.into(),
        AwsAv::Null(av) => rav.null = av.into(),
        AwsAv::S(av) => rav.s = av.into(),
        AwsAv::Ss(av) => rav.ss = av.into(),
        _ => unimplemented!(
            "A variant was added to aws_sdk_dynamodb::types::AttributeValue \
                and not implemented here: {av:?}",
        ),
    }

    rav
}
#
# _ = output;
# Ok(())
# }
```

[1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeNames.html
[2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeValues.html
[3]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.html
[4]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html
[5]: https://docs.rs/rusoto_dynamodb/
[`rusoto_dynamodb::AttributeValue`]: https://docs.rs/rusoto_dynamodb/latest/rusoto_dynamodb/struct.AttributeValue.html
*/

// Re-export the crates publicly exposed in our API
pub use ::aws_sdk_dynamodb;
pub use ::num;
pub use ::serde_json;

pub mod condition;
mod expression;
pub mod key;
pub mod operand;
pub mod path;
pub mod update;
pub mod value;

pub use expression::{Builder, Expression};
pub use path::Path;
pub use value::{Map, Num, Scalar, Set, Value};

/// This exists just for formatting the doc examples.
#[cfg(test)]
mod examples {
    #[allow(dead_code)]
    async fn example_query() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::{Expression, Num, Path};
        use aws_config::BehaviorVersion;
        use aws_sdk_dynamodb::Client;

        let client = Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await);

        let query_output = Expression::builder()
            .with_filter(
                "name"
                    .parse::<Path>()?
                    .attribute_exists()
                    .and("age".parse::<Path>()?.greater_than_or_equal(Num::new(2.5))),
            )
            .with_projection(["name", "age"])
            .with_key_condition("id".parse::<Path>()?.key().equal(Num::new(42)))
            .build()
            .query(&client)
            .table_name("people")
            .send()
            .await?;

        _ = query_output;
        Ok(())
    }

    #[allow(dead_code)]
    async fn example_rusoto() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::{Expression, Num, Path};
        use aws_sdk_dynamodb::{primitives::Blob, types::AttributeValue as AwsAv};
        use itermap::IterMap;
        use rusoto_core::Region;
        use rusoto_dynamodb::{AttributeValue as RusotoAv, DynamoDb, DynamoDbClient, QueryInput};

        let expression = Expression::builder()
            .with_filter(
                "name"
                    .parse::<Path>()?
                    .attribute_exists()
                    .and("age".parse::<Path>()?.greater_than_or_equal(Num::new(2.5))),
            )
            .with_projection(["name", "age"])
            .with_key_condition("id".parse::<Path>()?.key().equal(Num::new(42)))
            .build();

        let input = QueryInput {
            filter_expression: expression.filter_expression,
            projection_expression: expression.projection_expression,
            key_condition_expression: expression.key_condition_expression,
            expression_attribute_names: expression.expression_attribute_names,
            expression_attribute_values: expression
                .expression_attribute_values
                .map(|values| values.into_iter().map_values(convert_av).collect()),
            table_name: String::from("people"),
            ..QueryInput::default()
        };

        let output = DynamoDbClient::new(Region::UsEast1).query(input).await?;

        fn convert_av(av: AwsAv) -> RusotoAv {
            let mut rav = RusotoAv::default();

            match av {
                AwsAv::B(av) => rav.b = Some(av.into_inner().into()),
                AwsAv::Bool(av) => rav.bool = av.into(),
                AwsAv::Bs(av) => {
                    rav.bs = Some(
                        av.into_iter()
                            .map(Blob::into_inner)
                            .map(Into::into)
                            .collect(),
                    )
                }
                AwsAv::L(av) => rav.l = Some(av.into_iter().map(convert_av).collect()),
                AwsAv::M(av) => rav.m = Some(av.into_iter().map_values(convert_av).collect()),
                AwsAv::N(av) => rav.n = av.into(),
                AwsAv::Ns(av) => rav.ns = av.into(),
                AwsAv::Null(av) => rav.null = av.into(),
                AwsAv::S(av) => rav.s = av.into(),
                AwsAv::Ss(av) => rav.ss = av.into(),
                _ => unimplemented!(
                    "A variant was added to aws_sdk_dynamodb::types::AttributeValue \
                        and not implemented here: {av:?}",
                ),
            }

            rav
        }

        _ = output;
        Ok(())
    }
}
