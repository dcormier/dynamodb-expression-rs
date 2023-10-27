/*!
A package to help build DynamoDB filter and condition expressions in a type safe way.

// TODO: An example here.

*/

extern crate alloc;

// Re-export the crates publicly exposed in our API
pub use ::aws_sdk_dynamodb;
pub use ::num;

pub mod condition;
pub mod expression;
pub mod key;
mod name;
pub mod operand;
pub mod path;
pub mod update;
pub mod value;

pub use condition::Comparator;
pub use expression::Expression;
pub use name::{name, Name};
pub use value::{
    binary_set, binary_value, bool_value, null_value, num_set, num_value, ref_value, string_set,
    string_value,
};
