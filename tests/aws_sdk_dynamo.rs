use aws_sdk_dynamodb::{
    operation::scan::ScanInput,
    types::{AttributeValue, Put},
};
use dynamodb_expression::{name, string_value, Comparator::*};

#[test]
fn scan_input() {
    ScanInput::builder()
        .filter_expression(
            name("#name")
                .begins_with(":prefix")
                .and(name("#age").comparison(Ge, string_value(":min_age"))),
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
            name("#name")
                .attribute_not_exists()
                .or(name("#name").size().comparison(Eq, string_value(":zero"))),
        )
        .expression_attribute_names("#name", "name")
        .expression_attribute_values(":zero", AttributeValue::N(0.to_string()))
        .build();
}
