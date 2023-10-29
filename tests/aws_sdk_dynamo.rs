#[cfg(test)]
mod dynamodb;

use std::{collections::HashMap, future::Future, pin::Pin};

use aws_sdk_dynamodb::{
    error::SdkError,
    operation::query::QueryError,
    types::{AttributeValue, ReturnValue},
};
use pretty_assertions::{assert_eq, assert_ne};

use dynamodb_expression::{
    expression::Expression,
    key::key,
    path::Path,
    string_value,
    update::{Remove, Set, SetAction},
};

use crate::dynamodb::{
    debug::DebugList,
    item::{new_item, ATTR_ID, ATTR_LIST, ATTR_NUM, ATTR_STRING},
    partial_eq::PartialEqItem,
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

/// A name for a field that doesn't exist in generated item from [`new_item`] and [`fresh_item`].
const ATTR_NEW_FIELD: &str = "new_field";

async fn test_update(config: &Config) {
    let client = config.client().await;
    let item = fresh_item(config).await;
    assert_eq!(None, item.get(ATTR_NEW_FIELD));

    let update = Expression::new_with_update(Set::from_iter([
        SetAction::from(Path::from(ATTR_STRING).assign("abcdef")),
        Path::from(ATTR_NUM).math().sub(3.5).into(),
        Path::from(ATTR_LIST)
            .list_append()
            .before()
            .list(["A new value at the beginning"])
            .into(),
        // DynamoDB won't let you append to the same list twice in the same update expression.
        // Path::from(ATTR_LIST)
        //     .list_append()
        //     .list(["A new value at the end"])
        //     .into(),
        Path::from(ATTR_NEW_FIELD)
            .if_not_exists()
            .value("A new field")
            .into(),
    ]))
    .update_item(client)
    .set_key(item_key(&item).into());

    // println!("{:?}", update.as_input());

    update
        .table_name(&config.table_name)
        .send()
        .await
        .expect("Failed to update item");

    // Once more to add another item to the end of that list.
    // DynamoDB won't allow both in a single update expression.
    let updated_item = Expression::new_with_update(
        Path::from(ATTR_LIST)
            .list_append()
            .list(["A new value at the end"]),
    )
    .update_item(client)
    .set_key(item_key(&item).into())
    .table_name(&config.table_name)
    .return_values(ReturnValue::AllNew)
    .send()
    .await
    .expect("Failed to update item")
    .attributes
    .expect("Where is the item?");

    // println!("Got item: {:#?}", DebugItem(&after_update));

    assert_ne!(item.get(ATTR_STRING), updated_item.get(ATTR_STRING));
    assert_eq!(
        "abcdef",
        updated_item
            .get(ATTR_STRING)
            .map(AttributeValue::as_s)
            .expect("Field is missing")
            .expect("That field should be a String"),
        "Assigning a new value to the field didn't work"
    );
    assert_ne!(item.get(ATTR_NUM), updated_item.get(ATTR_NUM));
    assert_eq!(
        "38.5",
        updated_item
            .get(ATTR_NUM)
            .map(AttributeValue::as_n)
            .expect("Field is missing")
            .expect("That field should be a Number"),
        "Subtraction didn't work"
    );
    assert_eq!(
        "A new field",
        updated_item
            .get(ATTR_NEW_FIELD)
            .map(AttributeValue::as_s)
            .expect("Field is missing")
            .expect("The field should be a string"),
        "The new field was not added to the item as expected"
    );

    let updated_list = updated_item
        .get(ATTR_LIST)
        .map(AttributeValue::as_l)
        .expect("List is missing")
        .expect("The field should be a list");
    assert_eq!(
        item.get(ATTR_LIST)
            .map(AttributeValue::as_l)
            .expect("List is missing")
            .expect("The field should be a list")
            .len()
            + 2,
        updated_list.len(),
        "The list should have had two items added to it"
    );
    assert_eq!(
        Some(&AttributeValue::S("A new value at the beginning".into())),
        updated_list.first(),
        "List is missing the new value at the beginning: {:#?}",
        DebugList(updated_list)
    );
    assert_eq!(
        Some(&AttributeValue::S("A new value at the end".into())),
        updated_list.last(),
        "List is missing the new value at th end: {:#?}",
        DebugList(updated_list)
    );

    // Remove those two items we added to the list
    let update = Expression::new_with_update(
        [(ATTR_LIST, 0), (ATTR_LIST, (updated_list.len() - 1) as u32)]
            .into_iter()
            .collect::<Remove>(),
    )
    .update_item(client)
    .set_key(item_key(&item).into());

    println!("{:?}", update.as_input());

    let list_cleaned = update
        .table_name(&config.table_name)
        .return_values(ReturnValue::AllNew)
        .send()
        .await
        .expect("Failed to update item")
        // Grab the updated item
        .attributes
        .expect("Where is the item?")
        // Specifically, grab the list.
        .remove(ATTR_LIST)
        .map(|list| {
            if let AttributeValue::L(list) = list {
                list
            } else {
                panic!("The field should be a list")
            }
        })
        .expect("List is missing")
        .clone();

    assert_eq!(updated_list.len() - 2, list_cleaned.len());
    assert!(
        !list_cleaned.contains(&AttributeValue::S("A new value at the beginning".into())),
        "Value was not removed from the beginning. The list: {:#?}",
        DebugList(list_cleaned.iter()),
    );
    assert!(
        !list_cleaned.contains(&AttributeValue::S("A new value at the end".into())),
        "Value was not removed from the end. The list: {:#?}",
        DebugList(list_cleaned.iter()),
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

/// Deletes the item (if it exists) from [`new_item`] and inserts a new one.
/// Returns the inserted item.
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

/// Gets the item key for the given item.
fn item_key(item: &HashMap<String, AttributeValue>) -> HashMap<String, AttributeValue> {
    [(ATTR_ID.into(), item[ATTR_ID].clone())].into()
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
