#[test]
fn scan_input() {
    use aws_sdk_dynamodb::{operation::scan::ScanInput, types::AttributeValue};
    use dynamodb_expression::{ref_value, Path};
    use pretty_assertions::assert_eq;

    let scan_input = ScanInput::builder()
        .filter_expression(
            Path::name("#name")
                .begins_with(ref_value("prefix"))
                .and(Path::name("#age").greater_than_or_equal(ref_value("min_age"))),
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
}

#[test]
fn put() {
    use aws_sdk_dynamodb::types::{AttributeValue, Put};
    use dynamodb_expression::{ref_value, Path};
    use pretty_assertions::assert_eq;

    let put = Put::builder()
        .item("name", AttributeValue::S("Jane".into()))
        .condition_expression(
            Path::name("#name")
                .attribute_not_exists()
                .or(Path::name("#name").size().equal(ref_value("zero"))),
        )
        .expression_attribute_names("#name", "name")
        .expression_attribute_values(":zero", AttributeValue::N(0.to_string()))
        .table_name("people")
        .build()
        .unwrap();

    assert_eq!(
        Some("attribute_not_exists(#name) OR size(#name) = :zero"),
        put.condition_expression()
    );
}

#[test]
fn query() {
    use aws_sdk_dynamodb::{operation::query::QueryInput, types::AttributeValue};
    use dynamodb_expression::{expression::Expression, num_value, ref_value, Path};
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
            Path::name("#0")
                .attribute_exists()
                .and(Path::name("#1").greater_than_or_equal(ref_value("0"))),
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
            Path::name("name")
                .attribute_exists()
                .and(Path::name("age").greater_than_or_equal(num_value(2.5))),
        )
        .with_projection(["name", "age"])
        .with_key_condition(Path::name("id").key().equal(num_value(42)))
        .build()
        .to_query_input_builder()
        .table_name("the_table")
        .build()
        .unwrap();

    println!("{qi_3:#?}");

    // Each of these methods builds an equivalent `QueryInput`.
    assert_eq!(qi_1, qi_2);
    assert_eq!(qi_2, qi_3);
}

// This is here as the basis for the example in the readme and the top-level crate docs.
// Intentionally not marked as a test because this isn't expected to run on its own.
#[allow(dead_code, unused_variables)]
async fn query_example(
) -> Result<(), aws_sdk_dynamodb::error::SdkError<aws_sdk_dynamodb::operation::query::QueryError>> {
    use dynamodb_expression::{expression::Expression, num_value, Path};

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

    Ok(())
}
