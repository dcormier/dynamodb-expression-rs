/*!
A package to help build DynamoDB filter and condition expressions in a type safe way.

```
use aws_sdk_dynamodb::{
    operation::scan::ScanInput,
    types::{AttributeValue, Put},
};
use dynamodb_expression::{name, string_value, Comparator::*};

# fn main() {
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

Put::builder()
    .condition_expression(
        name("#name")
            .attribute_not_exists()
            .or(name("#name").size().comparison(Eq, string_value(":zero"))),
    )
    .expression_attribute_names("#name", "name")
    .expression_attribute_values(":zero", AttributeValue::N(0.to_string()))
    .build();
# }
```
*/

extern crate alloc;

pub mod aws_sdk_dynamodb;

pub mod condition;
pub mod key;
pub mod name;
pub mod operand;
pub mod value;

pub use condition::Comparator;
pub use name::{name, Name};
pub use value::{
    binary_set_value, binary_value, bool_value, null_value, num_set_value, num_value,
    string_set_value, string_value, ScalarValue, SetValue,
};
