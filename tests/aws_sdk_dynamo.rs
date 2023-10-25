#[cfg(test)]
mod dynamodb;

use std::{collections::HashMap, future::Future, pin::Pin};

use aws_sdk_dynamodb::{error::SdkError, operation::query::QueryError, types::AttributeValue};
use easy_error::ErrorExt;
use pretty_assertions::{assert_eq, assert_ne};

use dynamodb_expression::{
    expression::Expression,
    key::key,
    string_value,
    update::{
        set::{self, Assign, Math},
        Remove, Set, Update,
    },
};

use crate::dynamodb::{
    item::{new_item, ATTR_ID, ATTR_NUM},
    partial_eq::PartialEqItem,
};

use self::dynamodb::{
    item::ATTR_STRING,
    setup::{clean_table, delete_table},
    Config,
};

const ITEM_ID: &str = "sanity item";

#[tokio::test]
async fn query() {
    test("query", |config| Box::pin(test_query(config))).await;
}

async fn test_query(config: &Config) {
    let item = fresh_item(config).await;
    let got = get_item(config)
        .await
        .expect("Failed to query item")
        .expect("Where is the item?");

    assert_eq!(PartialEqItem(item), PartialEqItem(got));
}

#[tokio::test]
async fn update() {
    test("update", |config| Box::pin(test_update(config))).await;
}

async fn test_update(config: &Config) {
    let client = config.client().await;
    let item = fresh_item(config).await;
    let update = Expression::new_with_update(
        Set::from(Assign::new(ATTR_STRING, "abcdef")).and(Math::builder(ATTR_NUM).sub().num(3.5)),
        // .and(Remove::from("ATTR_")),
    )
    .update_item(client)
    .key(ATTR_ID, item[ATTR_ID].clone());

    println!("{:?}", update.as_input());

    update
        .table_name(&config.table_name)
        .send()
        .await
        .expect("Failed to update item");

    let after_update = get_item(config)
        .await
        .expect("Failed to get item")
        .expect("Where is the item?");

    assert_ne!(item.get(ATTR_STRING), after_update.get(ATTR_STRING));
    assert_eq!(
        "abcdef",
        after_update
            .get(ATTR_STRING)
            .map(AttributeValue::as_s)
            .expect("Should have gotten that field")
            .expect("That field should be a String"),
        "Assigning a new value to the field didn't work"
    );
    assert_ne!(item.get(ATTR_NUM), after_update.get(ATTR_NUM));
    assert_eq!(
        "38.5",
        after_update
            .get(ATTR_NUM)
            .map(AttributeValue::as_n)
            .expect("Should have gotten that field")
            .expect("That field should be a Number"),
        "Subtraction didn't work"
    );
}

/// Wraps a test function in code to set up and tear down the DynamoDB table.
///
/// The `name` value must be safe for use as a DynamoDB table name.
async fn test<F, T>(name: &str, test_fn: F) -> T
where
    F: FnOnce(&Config) -> Pin<Box<dyn Future<Output = T> + '_>>,
{
    let mut config = Config::new_local();
    config.table_name = format!("{}-{}", config.table_name, name);
    let config = config; // No longer mutable.
    let client = config.client().await;

    clean_table(client, &config.table_name)
        .await
        .expect("error creating table");

    let result = (test_fn)(&config).await;

    delete_table(client, &config.table_name)
        .await
        .expect("error deleting table");

    result
}

/// Deletes the item (if it exists) and inserts a new one. Returns the inserted item.
async fn fresh_item(config: &Config) -> HashMap<String, AttributeValue> {
    let item = new_item(ITEM_ID);

    config
        .client()
        .await
        .put_item()
        .table_name(&config.table_name)
        .set_item(Some(item.clone()))
        .send()
        .await
        .expect("Failed to put item");

    item
}

/// Gets the item from the configured table
async fn get_item(
    config: &Config,
) -> Result<Option<HashMap<String, AttributeValue>>, SdkError<QueryError>> {
    Expression::new_with_key_condition(key(ATTR_ID).equal(string_value(ITEM_ID)))
        .query(config.client().await)
        .table_name(config.table_name.clone())
        .send()
        .await
        .map(|resp| {
            let mut items = resp.items.expect("Should have found items");

            assert!(
                items.len() <= 1,
                "Should not have gotten more than one item"
            );

            items.pop()
        })
}
