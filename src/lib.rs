/*!
A crate to help build DynamoDB condition, filter, key condition, and update
expressions in a type-safe way, including using [expression attribute names][1]
and [expression attribute values][2].

[`Path`] represents a [DynamoDB document path][3], and has many methods for
building various expressions. [`Expression`] is the type to use for a [DynamoDB
expression][4].

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

From here, see [`Path`] for building updates, filters, or conditions, and then
[`Expression`] to turn those into DynamoDB expressions.

[1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeNames.html
[2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeValues.html
[3]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html#Expressions.Attributes.NestedElements.DocumentPathExamples
[4]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.html
*/

// Re-export the crates publicly exposed in our API
pub use ::aws_sdk_dynamodb;
pub use ::num;

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

/// Exists to assist with formatting doc examples
#[cfg(test)]
mod examples {
    #[expect(dead_code, reason = "Exists to assist with formatting doc examples")]
    async fn example_query() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use aws_config::BehaviorVersion;
        use aws_sdk_dynamodb::Client;

        use crate::{Expression, Num, Path};

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
}
