use std::error::Error;

#[test]
fn scan_input() -> Result<(), Box<dyn Error>> {
    use aws_sdk_dynamodb::{operation::scan::ScanInput, types::AttributeValue};
    use dynamodb_expression::{value::Ref, Path};
    use pretty_assertions::assert_eq;

    let scan_input = ScanInput::builder()
        .filter_expression(
            "#name"
                .parse::<Path>()?
                .begins_with(Ref::new("prefix"))
                .and(
                    "#age"
                        .parse::<Path>()?
                        .greater_than_or_equal(Ref::new("min_age")),
                ),
        )
        .expression_attribute_names("#name", "name")
        .expression_attribute_values(":prefix", AttributeValue::S("Wil".into()))
        .expression_attribute_names("#age", "age")
        .expression_attribute_values(":min_age", AttributeValue::N("25".into()))
        .build()
        .unwrap();

    assert_eq!(
        Some("begins_with(#name, :prefix) AND #age >= :min_age"),
        scan_input.filter_expression()
    );

    Ok(())
}

#[test]
fn put() -> Result<(), Box<dyn Error>> {
    use aws_sdk_dynamodb::types::{AttributeValue, Put};
    use dynamodb_expression::{value::Ref, Expression, Path, Scalar};
    use pretty_assertions::assert_eq;

    let put_1 = Put::builder()
        .condition_expression("attribute_not_exists(#0) OR size(#0) = :0")
        .expression_attribute_names("#0", "name")
        .expression_attribute_values(":0", AttributeValue::N(0.to_string()))
        .table_name("people")
        .item("name", AttributeValue::S("Jane".into()))
        .build()
        .unwrap();

    let put_2 = Put::builder()
        .condition_expression(
            "#0".parse::<Path>()?
                .attribute_not_exists()
                .or("#0".parse::<Path>()?.size().equal(Ref::new("0"))),
        )
        .expression_attribute_names("#0", "name")
        .expression_attribute_values(":0", AttributeValue::N(0.to_string()))
        .table_name("people")
        .item("name", AttributeValue::S("Jane".into()))
        .build()
        .unwrap();

    let put_3 = Expression::builder()
        .with_condition(
            "name"
                .parse::<Path>()?
                .attribute_not_exists()
                .or("name".parse::<Path>()?.size().equal(Scalar::new_num(0))),
        )
        .build()
        .to_put_builder()
        .table_name("people")
        .item("name", AttributeValue::S("Jane".into()))
        .build()
        .unwrap();

    assert_eq!(put_1, put_2);
    assert_eq!(put_2, put_3);

    Ok(())
}

#[test]
fn query() -> Result<(), Box<dyn Error>> {
    use aws_sdk_dynamodb::{operation::query::QueryInput, types::AttributeValue};
    use dynamodb_expression::{value::Ref, Expression, Num, Path};
    use pretty_assertions::assert_eq;

    // Building the `QueryInput` manually.
    let qi_1 = QueryInput::builder()
        .filter_expression("attribute_exists(#0) AND #1 >= :0")
        .projection_expression("#0, #1")
        .key_condition_expression("#2 = :1")
        .expression_attribute_names("#0", "name")
        .expression_attribute_names("#1", "age")
        .expression_attribute_names("#2", "id")
        .expression_attribute_values(":0", AttributeValue::N("2.5".into()))
        .expression_attribute_values(":1", AttributeValue::N("42".into()))
        .table_name("the_table")
        .build()
        .unwrap();

    println!("{qi_1:#?}");

    // Building the `QueryInput` using this crate to help with the filter expression.
    let qi_2 = QueryInput::builder()
        .filter_expression(
            Path::new_name("#0")
                .attribute_exists()
                .and(Path::new_name("#1").greater_than_or_equal(Ref::new("0"))),
        )
        .projection_expression("#0, #1")
        .key_condition_expression("#2 = :1")
        .expression_attribute_names("#0", "name")
        .expression_attribute_names("#1", "age")
        .expression_attribute_names("#2", "id")
        .expression_attribute_values(":0", AttributeValue::N("2.5".into()))
        .expression_attribute_values(":1", AttributeValue::N("42".into()))
        .table_name("the_table")
        .build()
        .unwrap();

    println!("{qi_2:#?}");

    // Building the `QueryInput` by leveraging this crate to its fullest.
    let qi_3: QueryInput = Expression::builder()
        .with_filter(
            "name"
                .parse::<Path>()?
                .attribute_exists()
                .and("age".parse::<Path>()?.greater_than_or_equal(Num::new(2.5))),
        )
        .with_projection(["name", "age"])
        .with_key_condition("id".parse::<Path>()?.key().equal(Num::new(42)))
        .build()
        .to_query_input_builder()
        .table_name("the_table")
        .build()
        .unwrap();

    println!("{qi_3:#?}");

    // Each of these methods builds an equivalent `QueryInput`.
    assert_eq!(qi_1, qi_2);
    assert_eq!(qi_2, qi_3);

    Ok(())
}

// This is here as the basis for the example in the readme and the top-level crate docs.
// Intentionally not marked as a test because this isn't expected to run on its own.
#[allow(dead_code)]
async fn query_example() -> Result<(), Box<dyn Error>> {
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
        .table_name("your_table")
        .send()
        .await?;

    _ = query_output;
    Ok(())
}
