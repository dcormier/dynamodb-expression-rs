/*!
A crate to help build DynamoDB condition, filter, key condition, and update
expressions in a type-safe way, including handling [expression attribute names][1]
and [expression attribute values][2].

[`Expression`] is the type to use for a [DynamoDB expression][2].
[`Path`] represents a [DynamoDB item attribute or document path][3], and has
many methods for building various expressions.

An example showing a how to use this crate to perform a query:

```no_run
# use aws_sdk_dynamodb::{error::SdkError, operation::query::QueryError};
#
# async fn example() -> Result<(), SdkError<QueryError>> {
use dynamodb_expression::{Expression, Num, Path};

let client = aws_sdk_dynamodb::Client::new(&aws_config::load_from_env().await);

let query_output = Expression::builder()
    .with_filter(
        Path::new_name("name")
            .attribute_exists()
            .and(Path::new_name("age").greater_than_or_equal(Num::new(2.5))),
    )
    .with_projection(["name", "age"])
    .with_key_condition(Path::new_name("id").key().equal(Num::new(42)))
    .build()
    .query(&client)
    .table_name("your_table")
    .send()
    .await?;
#
# Ok(())
# }
```

From here, see [`Expression`] and [`Path`] for more examples.

[1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeNames.html
[2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeValues.html
[3]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.html
[4]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html
*/

extern crate alloc;

// Re-export the crates publicly exposed in our API
pub use ::aws_sdk_dynamodb;
pub use ::num;

pub mod condition;
pub mod expression;
pub mod key;
pub mod operand;
pub mod path;
pub mod update;
pub mod value;

pub use expression::Expression;
pub use path::Path;
pub use value::{Map, Num, Scalar, Set, Value};
