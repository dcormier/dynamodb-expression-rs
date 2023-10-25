#[cfg(test)]
mod dynamodb;

use std::{collections::HashMap, future::Future, pin::Pin};

use aws_sdk_dynamodb::{error::SdkError, operation::query::QueryError, types::AttributeValue};
use itermap::IterMap;
use pretty_assertions::{assert_eq, assert_ne};

use dynamodb_expression::{
    expression::Expression,
    key::key,
    path::{Element, FieldIndex, Path},
    string_value,
    update::{
        set::{Append, Assign, IfNotExists, Math},
        Remove, Set, Update,
    },
};

use crate::dynamodb::{
    debug::DebugList,
    item::{new_item, ATTR_BLOB, ATTR_ID, ATTR_LIST, ATTR_NULL, ATTR_NUM},
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

/// A name for a field that doesn't exist in generated item from the helper functions.
const ATTR_NEW_FIELD: &str = "new_field";

async fn test_update(config: &Config) {
    let client = config.client().await;
    let item = fresh_item(config).await;
    let item_key = Some([(ATTR_ID.into(), item[ATTR_ID].clone())].into());

    assert_eq!(None, item.get(ATTR_NEW_FIELD));

    let update = Expression::new_with_update(
        Set::from(Assign::new(ATTR_STRING, "abcdef"))
            .and(Math::builder(ATTR_NUM).sub().num(3.5))
            .and(
                Append::builder(ATTR_LIST)
                    .before()
                    .list(["A new value at the beginning"]),
            )
            // .and(Append::builder(ATTR_LIST).list(["A new value at the end"]))
            .and(IfNotExists::builder(ATTR_NEW_FIELD).value("A new field")),
    )
    .update_item(client)
    .set_key(item_key.clone());

    // println!("{:?}", update.as_input());

    update
        .table_name(&config.table_name)
        .send()
        .await
        .expect("Failed to update item");

    // Once more to add another item to the end of that list.
    // DynamoDB won't allow both in a single update expression.
    Expression::new_with_update(Update::set(
        Append::builder(ATTR_LIST).list(["A new value at the end"]),
    ))
    .update_item(client)
    .set_key(item_key.clone())
    .table_name(&config.table_name)
    .send()
    .await
    .expect("Failed to update item");

    let after_update = get_item(config)
        .await
        .expect("Failed to get item")
        .expect("Where is the item?");

    // println!("Got item: {:#?}", DebugItem(&after_update));

    assert_ne!(item.get(ATTR_STRING), after_update.get(ATTR_STRING));
    assert_eq!(
        "abcdef",
        after_update
            .get(ATTR_STRING)
            .map(AttributeValue::as_s)
            .expect("Field is missing")
            .expect("That field should be a String"),
        "Assigning a new value to the field didn't work"
    );

    assert_ne!(item.get(ATTR_NUM), after_update.get(ATTR_NUM));
    assert_eq!(
        "38.5",
        after_update
            .get(ATTR_NUM)
            .map(AttributeValue::as_n)
            .expect("Field is missing")
            .expect("That field should be a Number"),
        "Subtraction didn't work"
    );

    assert_eq!(
        "A new field",
        after_update
            .get(ATTR_NEW_FIELD)
            .map(AttributeValue::as_s)
            .expect("Field is missing")
            .expect("The field should be a string"),
        "The new field was not added to the item as expected"
    );

    let list = after_update
        .get(ATTR_LIST)
        .map(AttributeValue::as_l)
        .expect("List is missing")
        .expect("The field should be a list");
    assert_eq!(
        Some(&AttributeValue::S("A new value at the beginning".into())),
        list.first(),
        "List is missing the new value at the beginning: {:#?}",
        DebugList(list)
    );
    assert_eq!(
        Some(&AttributeValue::S("A new value at the end".into())),
        list.last(),
        "List is missing the new value at th end: {:#?}",
        DebugList(list)
    );

    // Remove those two items we added to the list
    let update = Expression::new_with_update(
        // TODO: Make this easier.
        [(ATTR_LIST, 0), (ATTR_LIST, (list.len() - 1) as u32)]
            .into_iter()
            .map(FieldIndex::from)
            .collect::<Remove>(),
    )
    .update_item(client)
    .set_key(item_key.clone());

    println!("{:?}", update.as_input());

    update
        .table_name(&config.table_name)
        .send()
        .await
        .expect("Failed to update item");

    // TODO: Assert that the resulting stored item has the fields removed

    // // TODO: Need to be able to create a `Remove` by just `Remove::from("foo")`.
    // Remove::from(ATTR_BLOB)
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
