# dynamodb-expression

A Rust crate to help build DynamoDB [condition, filter](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html), [key condition](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Query.KeyConditionExpressions.html), and [update
expressions](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html) in a type-safe way, including handling [expression attribute names](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeNames.html) and [expression attribute values](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeValues.html).

[![Crates.io](https://img.shields.io/crates/v/dynamodb-expression.svg)](https://crates.io/crates/dynamodb-expression)
[![Docs.rs](https://docs.rs/dynamodb-expression/badge.svg)](https://docs.rs/dynamodb-expression/)

An example showing a how to use this crate to perform a query:

```rust
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
```

For more, see [the docs](https://docs.rs/dynamodb-expression/).
