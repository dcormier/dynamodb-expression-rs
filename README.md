# dynamodb-expression

A Rust crate to help build DynamoDB [condition, filter](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html), [key condition](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Query.KeyConditionExpressions.html), and [update
expressions](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.UpdateExpressions.html) in a type-safe way, including using [expression attribute names](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeNames.html) and [expression attribute values](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeValues.html).

[![Crates.io](https://img.shields.io/crates/v/dynamodb-expression.svg)](https://crates.io/crates/dynamodb-expression)
[![Docs.rs](https://docs.rs/dynamodb-expression/badge.svg)](https://docs.rs/dynamodb-expression/)

An example showing a how to use this crate to perform a query:

```rust
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
```

For more, see [the docs](https://docs.rs/dynamodb-expression/).

## Cargo features

This crate only uses `aws-sdk-dynamodb` for its types. By default it leaves the
SDK's default features on, which include the SDK's legacy `rustls` HTTP client
(hyper 0.14 / rustls 0.21). To leave that out, turn off default features and
choose the SDK features you need, either through this crate's pass-through
features or on your own `aws-sdk-dynamodb` dependency:

```toml
dynamodb-expression = { version = "0.2.0-beta.10", default-features = false }
aws-sdk-dynamodb = { version = "1", default-features = false, features = ["default-https-client", "rt-tokio"] }
```

Pass-through features: `aws-sdk-dynamodb-default` (on by default),
`behavior-version-latest`, `default-https-client`, `legacy-https-client`,
`rt-tokio`, and `rustls`.
