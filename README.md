# dynamodb_expression

A Rust crate to help build DynamoDB condition, filter, key condition, and update
expressions in a type-safe way.

[![Crates.io](https://img.shields.io/crates/v/dynamodb-expression.svg)](https://crates.io/crates/dynamodb-expression)
[![Docs.rs](https://docs.rs/dynamodb-expression/badge.svg)](https://docs.rs/dynamodb-expression/)

An example showing a how to use this crate to perform a query:

```rust
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
```
