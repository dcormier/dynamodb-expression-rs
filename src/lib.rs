/*!
A crate to help build DynamoDB condition, filter, key condition, and update
expressions in a type-safe way.

[`Expression`] is the type to use for a [DynamoDB expression][1].
[`Path`] represents a [DynamoDB item attribute or document path][2], and has
many methods for building various expressions.
See the integration tests for [querying] and [updating] as a starting place.

An example showing a how to use this crate to perform a query:

```no_run
use dynamodb_expression::{Expression, num_value, path::Path};

let client = aws_sdk_dynamodb::Client::new(&aws_config::load_from_env().await);

let query_output = Expression::builder()
    .with_filter(
        Path::name("name")
            .attribute_exists()
            .and(Path::name("age").greater_than_or_equal(num_value(2.5))),
    )
    .with_projection(["name", "age"])
    .with_key_condition(Path::name("id").key().equal(num_value(42)))
    .build()
    .query(&client)
    .table_name("your_table")
    .send()
    .await?;
#
# Ok(())
```

[1]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.html
[2]: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.Attributes.html
[querying]: https://github.com/dcormier/dynamodb-expression-rs/blob/b18bc1c/tests/aws_sdk_dynamo.rs#L480-L486
[updating]: https://github.com/dcormier/dynamodb-expression-rs/blob/b18bc1c/tests/aws_sdk_dynamo.rs#L52
*/

// TODO: An example here.

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

pub use condition::Comparator;
pub use expression::Expression;
pub use path::Path;
pub use value::{
    binary_set, binary_value, bool_value, null_value, num_set, num_value, ref_value, string_set,
    string_value,
};
