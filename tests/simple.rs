mod dynamodb;

use aws_sdk_dynamodb::{
    operation::scan::ScanInput,
    types::{AttributeValue, Put},
};
use pretty_assertions::assert_eq;

use dynamodb_expression::{
    expression::Expression, key::key, num_value, path::Path, ref_value, string_value, Comparator::*,
};

#[test]
fn scan_input() {
    ScanInput::builder()
        .filter_expression(
            Path::from("#name")
                .begins_with(":prefix")
                .and(Path::from("#age").greater_than_or_equal(string_value(":min_age"))),
        )
        .expression_attribute_names("#name", "name")
        .expression_attribute_values(":prefix", AttributeValue::S("Wil".into()))
        .expression_attribute_names("#age", "age")
        .expression_attribute_values(":min_age", AttributeValue::N("25".into()))
        .build()
        .unwrap();
}

#[test]
fn put() {
    Put::builder()
        .condition_expression(
            Path::from("#name")
                .attribute_not_exists()
                .or(Path::from("#name")
                    .size()
                    .comparison(Eq, string_value(":zero"))),
        )
        .expression_attribute_names("#name", "name")
        .expression_attribute_values(":zero", AttributeValue::N(0.to_string()))
        .build();
}

#[test]
fn query() {
    use aws_sdk_dynamodb::operation::query::QueryInput;

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
            Path::from("#0")
                .attribute_exists()
                .and(Path::from("#1").greater_than_or_equal(ref_value("0"))),
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
    let qi_3: QueryInput = Expression::new_with_filter(
        Path::from("name")
            .attribute_exists()
            .and(Path::from("age").greater_than_or_equal(num_value(2.5))),
    )
    .with_projection(["name", "age"])
    .with_key_condition(key("id").equal(num_value(42)))
    .to_query_input_builder()
    .table_name("the_table")
    .build()
    .unwrap();

    println!("{qi_3:#?}");

    // Each of these methods builds an equivalent `QueryInput`.
    assert_eq!(qi_1, qi_2);
    assert_eq!(qi_2, qi_3);
}
