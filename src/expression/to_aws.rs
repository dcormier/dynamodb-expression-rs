use aws_sdk_dynamodb::{
    operation::{
        delete_item::{
            builders::{DeleteItemFluentBuilder, DeleteItemInputBuilder},
            DeleteItemInput,
        },
        get_item::{
            builders::{GetItemFluentBuilder, GetItemInputBuilder},
            GetItemInput,
        },
        put_item::{
            builders::{PutItemFluentBuilder, PutItemInputBuilder},
            PutItemInput,
        },
        query::{
            builders::{QueryFluentBuilder, QueryInputBuilder},
            QueryInput,
        },
        scan::{
            builders::{ScanFluentBuilder, ScanInputBuilder},
            ScanInput,
        },
        update_item::{
            builders::{UpdateItemFluentBuilder, UpdateItemInputBuilder},
            UpdateItemInput,
        },
    },
    types::{
        builders::{
            ConditionCheckBuilder, DeleteBuilder, GetBuilder, KeysAndAttributesBuilder, PutBuilder,
            UpdateBuilder,
        },
        ConditionCheck, Delete, Get, KeysAndAttributes, Put, Update,
    },
    Client,
};

use super::Expression;

// Conversions to DynamoDB input builders

// TODO: Allow each of these impl blocks to be turned on/off with features?

// Put
impl Expression {
    /// Uses this [`Expression`] to create a [`PutBuilder`] with the following set:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_put_builder(&self) -> PutBuilder {
        Put::builder()
            .set_condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    /// Uses this [`Expression`] to set the following on a [`PutItemInputBuilder`]:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_put_item_input_builder(&self) -> PutItemInputBuilder {
        PutItemInput::builder()
            .set_condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    /// Uses this [`Expression`] to set the following on a [`PutItemFluentBuilder`]:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_put_item_fluent_builder(
        &self,
        builder: PutItemFluentBuilder,
    ) -> PutItemFluentBuilder {
        builder
            .set_condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    /// Sets up a `put_item` using the provided [`Client`] and uses this [`Expression`]
    /// to set the following on the [`PutItemFluentBuilder`]:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn put_item(&self, client: &Client) -> PutItemFluentBuilder {
        self.to_put_item_fluent_builder(client.put_item())
    }
}

// Get
impl Expression {
    /// Uses this [`Expression`] to create a [`GetBuilder`] with the following set:
    /// * Projection expression
    /// * Expression attribute names
    pub fn to_get_builder(&self) -> GetBuilder {
        Get::builder()
            .set_projection_expression(self.projection_expression())
            .set_expression_attribute_names(self.attribute_names())
    }

    /// Uses this [`Expression`] to set the following on a [`GetItemInputBuilder`]:
    /// * Projection expression
    /// * Expression attribute names
    pub fn to_get_item_input_builder(&self) -> GetItemInputBuilder {
        GetItemInput::builder()
            .set_projection_expression(self.projection_expression())
            .set_expression_attribute_names(self.attribute_names())
    }

    /// Uses this [`Expression`] to set the following on a [`GetItemFluentBuilder`]:
    /// * Projection expression
    /// * Expression attribute names
    pub fn to_get_item_fluent_builder(
        &self,
        builder: GetItemFluentBuilder,
    ) -> GetItemFluentBuilder {
        builder
            .set_projection_expression(self.projection_expression())
            .set_expression_attribute_names(self.attribute_names())
    }

    /// Sets up a `get_item` using the provided [`Client`] and uses this [`Expression`]
    /// to set the following on the [`GetItemFluentBuilder`]:
    /// * Projection expression
    /// * Expression attribute names
    pub fn get_item(&self, client: &Client) -> GetItemFluentBuilder {
        self.to_get_item_fluent_builder(client.get_item())
    }
}

// Update
impl Expression {
    /// Uses this [`Expression`] to create an [`UpdateBuilder`] with the following set:
    /// * Update expression
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_update_builder(&self) -> UpdateBuilder {
        Update::builder()
            .set_update_expression(self.update_expression())
            .set_condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    /// Uses this [`Expression`] to create an [`UpdateItemInputBuilder`] with the following set:
    /// * Update expression
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_update_item_input_builder(&self) -> UpdateItemInputBuilder {
        UpdateItemInput::builder()
            .set_update_expression(self.update_expression())
            .set_condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    /// Uses this [`Expression`] to set the following on an [`UpdateItemFluentBuilder`]:
    /// * Update expression
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_update_item_fluent_builder(
        &self,
        builder: UpdateItemFluentBuilder,
    ) -> UpdateItemFluentBuilder {
        builder
            .set_update_expression(self.update_expression())
            .set_condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    /// Sets up an `update_item` using the provided [`Client`] and uses this [`Expression`]
    /// to set the following on the [`UpdateItemFluentBuilder`]:
    /// * Update expression
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn update_item(&self, client: &Client) -> UpdateItemFluentBuilder {
        self.to_update_item_fluent_builder(client.update_item())
    }
}

// Delete
impl Expression {
    /// Uses this [`Expression`] to create a [`DeleteBuilder`] with the following set:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_delete_builder(&self) -> DeleteBuilder {
        Delete::builder()
            .set_condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    /// Uses this [`Expression`] to set the following on a [`DeleteItemInputBuilder`]:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_delete_item_input_builder(&self) -> DeleteItemInputBuilder {
        DeleteItemInput::builder()
            .set_condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    /// Uses this [`Expression`] to set the following on a [`DeleteItemFluentBuilder`]:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_delete_item_fluent_builder(
        &self,
        builder: DeleteItemFluentBuilder,
    ) -> DeleteItemFluentBuilder {
        builder
            .set_condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    /// Sets up a `delete_item` using the provided [`Client`] and uses this [`Expression`]
    /// to set the following on the [`DeleteItemFluentBuilder`]:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn delete_item(&self, client: &Client) -> DeleteItemFluentBuilder {
        self.to_delete_item_fluent_builder(client.delete_item())
    }
}

// Query
impl Expression {
    /// Uses this [`Expression`] to create a [`QueryInputBuilder`] with the following set:
    /// * Key condition expression
    /// * Filter expression
    /// * Projection expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_query_input_builder(&self) -> QueryInputBuilder {
        QueryInput::builder()
            .set_key_condition_expression(self.key_condition_expression())
            .set_filter_expression(self.filter_expression())
            .set_projection_expression(self.projection_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    /// Uses this [`Expression`] to set the following on a [`QueryFluentBuilder`]:
    /// * Key condition expression
    /// * Filter expression
    /// * Projection expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_query_fluent_builder(&self, builder: QueryFluentBuilder) -> QueryFluentBuilder {
        builder
            .set_key_condition_expression(self.key_condition_expression())
            .set_filter_expression(self.filter_expression())
            .set_projection_expression(self.projection_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    /// Sets up a `query` using the provided [`Client`] and uses this [`Expression`]
    /// to set the following on the [`QueryFluentBuilder`]:
    /// * Key condition expression
    /// * Filter expression
    /// * Projection expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn query(&self, client: &Client) -> QueryFluentBuilder {
        self.to_query_fluent_builder(client.query())
    }
}

// Scan
impl Expression {
    /// Uses this [`Expression`] to create a [`ScanInputBuilder`] with the following set:
    /// * Filter expression
    /// * Projection expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_scan_input_builder(&self) -> ScanInputBuilder {
        ScanInput::builder()
            .set_filter_expression(self.filter_expression())
            .set_projection_expression(self.projection_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    /// Uses this [`Expression`] to set the following on a [`ScanFluentBuilder`]:
    /// * Filter expression
    /// * Projection expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_scan_fluent_builder(&self, builder: ScanFluentBuilder) -> ScanFluentBuilder {
        builder
            .set_filter_expression(self.filter_expression())
            .set_projection_expression(self.projection_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    /// Sets up a `scan` using the provided [`Client`] and uses this [`Expression`]
    /// to set the following on the [`ScanFluentBuilder`]:
    /// * Filter expression
    /// * Projection expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn scan(&self, client: &Client) -> ScanFluentBuilder {
        self.to_scan_fluent_builder(client.scan())
    }
}

impl Expression {
    // TODO: This ultimately ends up being a part of a `BatchGetItemInput`.
    //       See how that gets used in practice.
    // https://docs.rs/aws-sdk-dynamodb/latest/aws_sdk_dynamodb/operation/batch_get_item/builders/struct.BatchGetItemInputBuilder.html#method.request_items
    //
    // TODO: If feature flags are used for these blocks, should this be grouped
    //       with the other methods related to `get`?
    // ----
    /// Uses this [`Expression`] to create a [`KeysAndAttributesBuilder`] with the following set:
    /// * Projection expression
    /// * Expression attribute names
    pub fn to_keys_and_attributes_builder(&self) -> KeysAndAttributesBuilder {
        KeysAndAttributes::builder()
            .set_projection_expression(self.projection_expression())
            .set_expression_attribute_names(self.attribute_names())
    }

    // TODO: This ultimately ends up being a part of a `TransactWriteItem`.
    // See how that gets used in practice.
    // https://docs.rs/aws-sdk-dynamodb/latest/aws_sdk_dynamodb/types/builders/struct.TransactWriteItemBuilder.html#method.condition_check
    //
    // TODO: If feature flags are used for these blocks, should this be grouped
    //       with the other methods related to `put_item` (or is it `update`)?
    // ----
    /// Uses this [`Expression`] to create a [`ConditionCheckBuilder`] with the following set:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_condition_check_builder(&self) -> ConditionCheckBuilder {
        ConditionCheck::builder()
            .set_condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::{assert_eq, assert_ne};

    use crate::{key::Key, num_value, path::Path};

    use super::Expression;

    #[test]
    fn scan_input() {
        let expression = Expression::new_with_filter(
            "name".parse::<Path>().unwrap().begins_with("prefix").and(
                "age"
                    .parse::<Path>()
                    .unwrap()
                    .greater_than_or_equal(num_value(25)),
            ),
        )
        .with_projection(["name", "age"]);
        assert_eq!(None, expression.condition);

        let filter = expression
            .filter_expression()
            .expect("A filter expression should be set");
        assert_eq!("begins_with(#0, :0) AND #1 >= :1", filter);
        println!("Filter: {filter}");

        let projection = expression
            .projection_expression()
            .expect("A projection expression should be set");
        assert_eq!("#0, #1", projection);
        println!("Projection: {projection}");

        println!("Names: {:?}", expression.attribute_names());
        println!("Values: {:?}", expression.attribute_values());

        let si = expression.to_scan_input_builder().build().unwrap();

        println!("{si:#?}");
    }

    #[test]
    fn query_input() {
        let expression = Expression::new_with_filter(
            "name".parse::<Path>().unwrap().attribute_exists().and(
                "age"
                    .parse::<Path>()
                    .unwrap()
                    .greater_than_or_equal(num_value(2.5)),
            ),
        )
        .with_projection(["name", "age"])
        .with_key_condition(Key::from("id").equal(num_value(42)));
        assert_eq!(None, expression.condition);

        let filter = expression
            .filter_expression()
            .expect("A filter expression should be set");
        assert_ne!("", filter);
        println!("{filter}");

        let projection = expression
            .projection_expression()
            .expect("A projection expression should be set");
        assert_eq!("#0, #1", projection);
        println!("Projection: {projection}");

        println!("Names: {:?}", expression.names);
        println!("Values: {:?}", expression.values);

        let qi = expression.to_query_input_builder().build().unwrap();

        println!("{qi:#?}");
    }

    #[test]
    fn update() {
        let expression = Expression::new_with_condition(
            "name".parse::<Path>().unwrap().attribute_exists().and(
                "age"
                    .parse::<Path>()
                    .unwrap()
                    .greater_than_or_equal(num_value(25)),
            ),
        );

        let condition = expression
            .condition_expression()
            .expect("A condition expression should be set");
        assert_ne!("", condition);
        println!("{condition}");

        println!("Names: {:?}", expression.names);
        println!("Values: {:?}", expression.values);

        let update = expression.to_update_builder().build();

        println!("{update:#?}");
    }
}
