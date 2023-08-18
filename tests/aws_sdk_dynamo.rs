use aws_sdk_dynamodb::{
    operation::scan::ScanInput,
    types::{AttributeValue, Put},
};
use dynamodb_expression::{attribute_not_exists, begins_with, cmp, size, Comparator::*};

#[test]
fn scan_input() {
    ScanInput::builder()
        .filter_expression(begins_with("#name", ":prefix").and(cmp("#age", GE, ":min_age")))
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
        .condition_expression(attribute_not_exists("#name").or(cmp(size("#name"), EQ, ":zero")))
        .expression_attribute_names("#name", "name")
        .expression_attribute_values(":zero", AttributeValue::N(0.to_string()))
        .build();
}
