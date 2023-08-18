/*!
A package to help build DynamoDB filter and condition expressions in a type safe way.

```
use aws_sdk_dynamodb::{
    operation::scan::ScanInput,
    types::{AttributeValue, Put},
};
use dynamodb_expression::{attribute_not_exists, begins_with, cmp, size, Comparator::*};

# fn main() {
ScanInput::builder()
    .filter_expression(begins_with("#name", ":prefix").and(cmp("#age", GE, ":min_age")))
    .expression_attribute_names("#name", "name")
    .expression_attribute_values(":prefix", AttributeValue::S("Wil".into()))
    .expression_attribute_names("#age", "age")
    .expression_attribute_values(":min_age", AttributeValue::N("25".into()))
    .build()
    .unwrap();

Put::builder()
    .condition_expression(attribute_not_exists("#name").or(cmp(size("#name"), EQ, ":zero")))
    .expression_attribute_names("#name", "name")
    .expression_attribute_values(":zero", AttributeValue::N(0.to_string()))
    .build();
# }
```
*/

// https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.OperatorsAndFunctions.html#Expressions.OperatorsAndFunctions.Syntax

pub mod attribute_type;
pub mod comparator;
pub mod expression;
pub mod function;
pub mod not;
pub mod parenthetical;

pub use attribute_type::AttributeType;
pub use comparator::Comparator;
pub use expression::{between, cmp, in_};

pub use function::{
    attribute_exists, attribute_not_exists, attribute_type, begins_with, contains, size,
};
