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

/// Methods related to [`PutItem` operations][1].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_PutItem.html
impl Expression {
    /// Uses this [`Expression`] to create a [`PutBuilder`] with the following set:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_put_builder(self) -> PutBuilder {
        Put::builder()
            .set_condition_expression(self.condition_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
            .set_expression_attribute_values(self.expression_attribute_values)
    }

    /// Uses this [`Expression`] to set the following on a [`PutItemInputBuilder`]:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_put_item_input_builder(self) -> PutItemInputBuilder {
        PutItemInput::builder()
            .set_condition_expression(self.condition_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
            .set_expression_attribute_values(self.expression_attribute_values)
    }

    /// Uses this [`Expression`] to set the following on a [`PutItemFluentBuilder`]
    /// before returning it:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_put_item_fluent_builder(self, builder: PutItemFluentBuilder) -> PutItemFluentBuilder {
        builder
            .set_condition_expression(self.condition_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
            .set_expression_attribute_values(self.expression_attribute_values)
    }

    /// Sets up a [`put_item`][1] using the provided [`Client`] and uses this [`Expression`]
    /// to set the following on the [`PutItemFluentBuilder`] before returning it:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example_put_item() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// use aws_config::BehaviorVersion;
    /// use aws_sdk_dynamodb::{types::AttributeValue, Client};
    /// use dynamodb_expression::{Expression, Path};
    ///
    /// let client = Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await);
    ///
    /// let output = Expression::builder()
    ///     .with_condition(Path::new_name("name").attribute_not_exists())
    ///     .build()
    ///     .put_item(&client)
    ///     .table_name("people")
    ///     .item("name", AttributeValue::S(String::from("Jill")))
    ///     .item("age", AttributeValue::N(40.to_string()))
    ///     .send()
    ///     .await?;
    /// #
    /// # _ = output;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_PutItem.html
    pub fn put_item(self, client: &Client) -> PutItemFluentBuilder {
        self.to_put_item_fluent_builder(client.put_item())
    }
}

/// Methods related to [`GetItem` operations][1].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_GetItem.html
impl Expression {
    /// Uses this [`Expression`] to create a [`GetBuilder`] with the following set:
    /// * Projection expression
    /// * Expression attribute names
    pub fn to_get_builder(self) -> GetBuilder {
        Get::builder()
            .set_projection_expression(self.projection_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
    }

    /// Uses this [`Expression`] to set the following on a [`GetItemInputBuilder`]:
    /// * Projection expression
    /// * Expression attribute names
    pub fn to_get_item_input_builder(self) -> GetItemInputBuilder {
        GetItemInput::builder()
            .set_projection_expression(self.projection_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
    }

    /// Uses this [`Expression`] to set the following on a [`GetItemFluentBuilder`]
    /// before returning it:
    /// * Projection expression
    /// * Expression attribute names
    pub fn to_get_item_fluent_builder(self, builder: GetItemFluentBuilder) -> GetItemFluentBuilder {
        builder
            .set_projection_expression(self.projection_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
    }

    /// Sets up a [`get_item`][1] using the provided [`Client`] and uses this [`Expression`]
    /// to set the following on the [`GetItemFluentBuilder`] before returning it:
    /// * Projection expression
    /// * Expression attribute names
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example_get_item() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// use aws_config::BehaviorVersion;
    /// use aws_sdk_dynamodb::{types::AttributeValue, Client};
    /// use dynamodb_expression::Expression;
    ///
    /// let client = Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await);
    ///
    /// let output = Expression::builder()
    ///     .with_projection(["name", "age"])
    ///     .build()
    ///     .get_item(&client)
    ///     .table_name("people")
    ///     .key("id", AttributeValue::N(42.to_string()))
    ///     .send()
    ///     .await?;
    /// #
    /// # _ = output;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_GetItem.html
    pub fn get_item(self, client: &Client) -> GetItemFluentBuilder {
        self.to_get_item_fluent_builder(client.get_item())
    }
}

/// Methods related to [`UpdateItem` operations][1].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_UpdateItem.html
impl Expression {
    /// Uses this [`Expression`] to create an [`UpdateBuilder`] with the following set:
    /// * Update expression
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_update_builder(self) -> UpdateBuilder {
        Update::builder()
            .set_update_expression(self.update_expression)
            .set_condition_expression(self.condition_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
            .set_expression_attribute_values(self.expression_attribute_values)
    }

    /// Uses this [`Expression`] to create an [`UpdateItemInputBuilder`] with the following set:
    /// * Update expression
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_update_item_input_builder(self) -> UpdateItemInputBuilder {
        UpdateItemInput::builder()
            .set_update_expression(self.update_expression)
            .set_condition_expression(self.condition_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
            .set_expression_attribute_values(self.expression_attribute_values)
    }

    /// Uses this [`Expression`] to set the following on an [`UpdateItemFluentBuilder`]
    /// before returning it:
    /// * Update expression
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_update_item_fluent_builder(
        self,
        builder: UpdateItemFluentBuilder,
    ) -> UpdateItemFluentBuilder {
        builder
            .set_update_expression(self.update_expression)
            .set_condition_expression(self.condition_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
            .set_expression_attribute_values(self.expression_attribute_values)
    }

    /// Sets up an [`update_item`][1] using the provided [`Client`] and uses this [`Expression`]
    /// to set the following on the [`UpdateItemFluentBuilder`] before returning it:
    /// * Update expression
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example_update_item() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>
    /// # {
    /// use aws_config::BehaviorVersion;
    /// use aws_sdk_dynamodb::{types::AttributeValue, Client};
    /// use dynamodb_expression::{Expression, Num, Path};
    ///
    /// let client = Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await);
    ///
    /// let age = Path::new_name("age");
    /// let output = Expression::builder()
    ///     .with_condition(age.clone().equal(Num::new(40)))
    ///     .with_update(age.math().add(1))
    ///     .build()
    ///     .update_item(&client)
    ///     .table_name("people")
    ///     .key("name", AttributeValue::S(String::from("Jack")))
    ///     .send()
    ///     .await?;
    /// #
    /// # _ = output;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_UpdateItem.html
    pub fn update_item(self, client: &Client) -> UpdateItemFluentBuilder {
        self.to_update_item_fluent_builder(client.update_item())
    }
}

/// Methods related to [`DeleteItem` operations][1].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_DeleteItem.html
impl Expression {
    /// Uses this [`Expression`] to create a [`DeleteBuilder`] with the following set:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_delete_builder(self) -> DeleteBuilder {
        Delete::builder()
            .set_condition_expression(self.condition_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
            .set_expression_attribute_values(self.expression_attribute_values)
    }

    /// Uses this [`Expression`] to set the following on a [`DeleteItemInputBuilder`]:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_delete_item_input_builder(self) -> DeleteItemInputBuilder {
        DeleteItemInput::builder()
            .set_condition_expression(self.condition_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
            .set_expression_attribute_values(self.expression_attribute_values)
    }

    /// Uses this [`Expression`] to set the following on a [`DeleteItemFluentBuilder`]
    /// before returning it:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_delete_item_fluent_builder(
        self,
        builder: DeleteItemFluentBuilder,
    ) -> DeleteItemFluentBuilder {
        builder
            .set_condition_expression(self.condition_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
            .set_expression_attribute_values(self.expression_attribute_values)
    }

    /// Sets up a [`delete_item`][1] using the provided [`Client`] and uses this [`Expression`]
    /// to set the following on the [`DeleteItemFluentBuilder`] before returning it:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example_delete_item() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>
    /// # {
    /// use aws_config::BehaviorVersion;
    /// use aws_sdk_dynamodb::{types::AttributeValue, Client};
    /// use dynamodb_expression::{Expression, Num, Path};
    ///
    /// let client = Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await);
    ///
    /// let output = Expression::builder()
    ///     .with_condition(Path::new_name("age").less_than(Num::new(20)))
    ///     .build()
    ///     .delete_item(&client)
    ///     .table_name("people")
    ///     .key("name", AttributeValue::S(String::from("Jack")))
    ///     .send()
    ///     .await?;
    /// #
    /// # _ = output;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_DeleteItem.html
    pub fn delete_item(self, client: &Client) -> DeleteItemFluentBuilder {
        self.to_delete_item_fluent_builder(client.delete_item())
    }
}

/// Methods related to [`Query` operations][1].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Query.html
impl Expression {
    /// Uses this [`Expression`] to create a [`QueryInputBuilder`] with the following set:
    /// * Key condition expression
    /// * Filter expression
    /// * Projection expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_query_input_builder(self) -> QueryInputBuilder {
        QueryInput::builder()
            .set_key_condition_expression(self.key_condition_expression)
            .set_filter_expression(self.filter_expression)
            .set_projection_expression(self.projection_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
            .set_expression_attribute_values(self.expression_attribute_values)
    }

    /// Uses this [`Expression`] to set the following on a [`QueryFluentBuilder`]
    /// before returning it:
    /// * Key condition expression
    /// * Filter expression
    /// * Projection expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_query_fluent_builder(self, builder: QueryFluentBuilder) -> QueryFluentBuilder {
        builder
            .set_key_condition_expression(self.key_condition_expression)
            .set_filter_expression(self.filter_expression)
            .set_projection_expression(self.projection_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
            .set_expression_attribute_values(self.expression_attribute_values)
    }

    /// Sets up a [`query`][1] using the provided [`Client`] and uses this [`Expression`]
    /// to set the following on the [`QueryFluentBuilder`] before returning it:
    /// * Key condition expression
    /// * Filter expression
    /// * Projection expression
    /// * Expression attribute names
    /// * Expression attribute values
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example_query() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// use aws_config::BehaviorVersion;
    /// use aws_sdk_dynamodb::Client;
    /// use dynamodb_expression::{Expression, Num, Path};
    ///
    /// let client = Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await);
    ///
    /// let output = Expression::builder()
    ///     .with_filter(
    ///         Path::new_name("name")
    ///             .attribute_exists()
    ///             .and(Path::new_name("age").greater_than_or_equal(Num::new(25))),
    ///     )
    ///     .with_projection(["name", "age"])
    ///     .with_key_condition(Path::new_name("id").key().equal(Num::new(42)))
    ///     .build()
    ///     .query(&client)
    ///     .table_name("people")
    ///     .send()
    ///     .await?;
    /// #
    /// # _ = output;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Query.html
    pub fn query(self, client: &Client) -> QueryFluentBuilder {
        self.to_query_fluent_builder(client.query())
    }
}

/// Methods related to [`Scan` operations][1].
///
/// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Scan.html
impl Expression {
    /// Uses this [`Expression`] to create a [`ScanInputBuilder`] with the following set:
    /// * Filter expression
    /// * Projection expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_scan_input_builder(self) -> ScanInputBuilder {
        ScanInput::builder()
            .set_filter_expression(self.filter_expression)
            .set_projection_expression(self.projection_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
            .set_expression_attribute_values(self.expression_attribute_values)
    }

    /// Uses this [`Expression`] to set the following on a [`ScanFluentBuilder`]
    /// before returning it:
    /// * Filter expression
    /// * Projection expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_scan_fluent_builder(self, builder: ScanFluentBuilder) -> ScanFluentBuilder {
        builder
            .set_filter_expression(self.filter_expression)
            .set_projection_expression(self.projection_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
            .set_expression_attribute_values(self.expression_attribute_values)
    }

    /// Sets up a [`scan`][1] using the provided [`Client`] and uses this [`Expression`]
    /// to set the following on the [`ScanFluentBuilder`] before returning it:
    /// * Filter expression
    /// * Projection expression
    /// * Expression attribute names
    /// * Expression attribute values
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example_scan() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    /// use aws_config::BehaviorVersion;
    /// use aws_sdk_dynamodb::Client;
    /// use dynamodb_expression::{Expression, Num, Path};
    ///
    /// let client = Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await);
    ///
    /// let output = Expression::builder()
    ///     .with_filter(Path::new_name("age").greater_than_or_equal(Num::new(25)))
    ///     .with_projection(["name", "age"])
    ///     .build()
    ///     .scan(&client)
    ///     .table_name("people")
    ///     .send()
    ///     .await?;
    /// #
    /// # _ = output;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [1]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Scan.html
    pub fn scan(self, client: &Client) -> ScanFluentBuilder {
        self.to_scan_fluent_builder(client.scan())
    }
}

impl Expression {
    // TODO: This ultimately ends up being a part of a `BatchGetItemInput`.
    //       See how that gets used in practice.
    // https://docs.rs/aws-sdk-dynamodb/latest/aws_sdk_dynamodb/operation/batch_get_item/builders/struct.BatchGetItemInputBuilder.html#method.request_items
    // ----
    /// Uses this [`Expression`] to create a [`KeysAndAttributesBuilder`] with the following set:
    /// * Projection expression
    /// * Expression attribute names
    pub fn to_keys_and_attributes_builder(self) -> KeysAndAttributesBuilder {
        KeysAndAttributes::builder()
            .set_projection_expression(self.projection_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
    }

    // TODO: This ultimately ends up being a part of a `TransactWriteItem`.
    //       See how that gets used in practice.
    // https://docs.rs/aws-sdk-dynamodb/latest/aws_sdk_dynamodb/types/builders/struct.TransactWriteItemBuilder.html#method.condition_check
    // ----
    /// Uses this [`Expression`] to create a [`ConditionCheckBuilder`] with the following set:
    /// * Condition expression
    /// * Expression attribute names
    /// * Expression attribute values
    pub fn to_condition_check_builder(self) -> ConditionCheckBuilder {
        ConditionCheck::builder()
            .set_condition_expression(self.condition_expression)
            .set_expression_attribute_names(self.expression_attribute_names)
            .set_expression_attribute_values(self.expression_attribute_values)
    }
}

#[cfg(test)]
mod test {

    /// Exists to simplify the doc examples
    #[allow(dead_code)]
    async fn example_put_item() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        use crate::{Expression, Path};
        use aws_config::BehaviorVersion;
        use aws_sdk_dynamodb::{types::AttributeValue, Client};

        let client = Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await);

        let output = Expression::builder()
            .with_condition(Path::new_name("name").attribute_not_exists())
            .build()
            .put_item(&client)
            .table_name("people")
            .item("name", AttributeValue::S(String::from("Jill")))
            .item("age", AttributeValue::N(40.to_string()))
            .send()
            .await?;

        _ = output;
        Ok(())
    }

    /// Exists to simplify the doc examples
    #[allow(dead_code)]
    async fn example_get_item() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        use crate::Expression;
        use aws_config::BehaviorVersion;
        use aws_sdk_dynamodb::{types::AttributeValue, Client};

        let client = Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await);

        let output = Expression::builder()
            .with_projection(["name", "age"])
            .build()
            .get_item(&client)
            .table_name("people")
            .key("id", AttributeValue::N(42.to_string()))
            .send()
            .await?;

        _ = output;
        Ok(())
    }

    /// Exists to simplify the doc examples
    #[allow(dead_code)]
    async fn example_scan() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        use crate::{Expression, Num, Path};
        use aws_config::BehaviorVersion;
        use aws_sdk_dynamodb::Client;

        let client = Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await);

        let output = Expression::builder()
            .with_filter(Path::new_name("age").greater_than_or_equal(Num::new(25)))
            .with_projection(["name", "age"])
            .build()
            .scan(&client)
            .table_name("people")
            .send()
            .await?;

        _ = output;
        Ok(())
    }

    /// Exists to simplify the doc examples
    #[allow(dead_code)]
    async fn example_query() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        use crate::{Expression, Num, Path};
        use aws_config::BehaviorVersion;
        use aws_sdk_dynamodb::Client;

        let client = Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await);

        let output = Expression::builder()
            .with_filter(
                Path::new_name("name")
                    .attribute_exists()
                    .and(Path::new_name("age").greater_than_or_equal(Num::new(25))),
            )
            .with_projection(["name", "age"])
            .with_key_condition(Path::new_name("id").key().equal(Num::new(42)))
            .build()
            .query(&client)
            .table_name("people")
            .send()
            .await?;

        _ = output;
        Ok(())
    }

    /// Exists to simplify the doc examples
    #[allow(dead_code)]
    async fn example_update_item() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>
    {
        use crate::{Expression, Num, Path};
        use aws_config::BehaviorVersion;
        use aws_sdk_dynamodb::{types::AttributeValue, Client};

        let client = Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await);

        let age = Path::new_name("age");
        let output = Expression::builder()
            .with_condition(age.clone().equal(Num::new(40)))
            .with_update(age.math().add(1))
            .build()
            .update_item(&client)
            .table_name("people")
            .key("name", AttributeValue::S(String::from("Jack")))
            .send()
            .await?;

        _ = output;
        Ok(())
    }

    /// Exists to simplify the doc examples
    #[allow(dead_code)]
    async fn example_delete_item() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>
    {
        use crate::{Expression, Num, Path};
        use aws_config::BehaviorVersion;
        use aws_sdk_dynamodb::{types::AttributeValue, Client};

        let client = Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await);

        let output = Expression::builder()
            .with_condition(Path::new_name("age").less_than(Num::new(20)))
            .build()
            .delete_item(&client)
            .table_name("people")
            .key("name", AttributeValue::S(String::from("Jack")))
            .send()
            .await?;

        _ = output;
        Ok(())
    }

    #[test]
    fn scan_input() {
        use crate::{Expression, Num, Path};
        use pretty_assertions::assert_eq;

        let expression = Expression::builder()
            .with_filter(
                "name".parse::<Path>().unwrap().begins_with("prefix").and(
                    "age"
                        .parse::<Path>()
                        .unwrap()
                        .greater_than_or_equal(Num::new(25)),
                ),
            )
            .with_projection(["name", "age"])
            .build();
        assert_eq!(None, expression.condition_expression);

        let filter = expression
            .filter_expression
            .as_deref()
            .expect("A filter expression should be set");
        assert_eq!("begins_with(#0, :0) AND #1 >= :1", filter);
        println!("Filter: {filter}");

        let projection = expression
            .projection_expression
            .as_deref()
            .expect("A projection expression should be set");
        assert_eq!("#0, #1", projection);
        println!("Projection: {projection}");

        println!("Names: {:?}", expression.expression_attribute_names);
        println!("Values: {:?}", expression.expression_attribute_values);

        let si = expression.to_scan_input_builder().build().unwrap();

        println!("{si:#?}");
    }

    #[test]
    fn query_input() {
        use crate::{key::Key, path::Name, Expression, Num, Path};
        use pretty_assertions::{assert_eq, assert_ne};

        let expression = Expression::builder()
            .with_filter(
                Path::from(Name::from("name"))
                    .attribute_exists()
                    .and(Path::from(Name::from("age")).greater_than_or_equal(Num::new(2.5))),
            )
            .with_projection(["name", "age"])
            .with_key_condition(Key::from(Name::from("id")).equal(Num::new(42)))
            .build();
        assert_eq!(None, expression.condition_expression);

        let filter = expression
            .filter_expression
            .as_deref()
            .expect("A filter expression should be set");
        assert_ne!("", filter);
        println!("{filter}");

        let projection = expression
            .projection_expression
            .as_deref()
            .expect("A projection expression should be set");
        assert_eq!("#0, #1", projection);
        println!("Projection: {projection}");

        println!("Names: {:?}", expression.expression_attribute_names);
        println!("Values: {:?}", expression.expression_attribute_values);

        let qi = expression.to_query_input_builder().build().unwrap();

        println!("{qi:#?}");
    }

    #[test]
    fn update() {
        use crate::{Expression, Num, Path};
        use pretty_assertions::assert_ne;

        let expression = Expression::builder()
            .with_condition(
                "name".parse::<Path>().unwrap().attribute_exists().and(
                    "age"
                        .parse::<Path>()
                        .unwrap()
                        .greater_than_or_equal(Num::new(25)),
                ),
            )
            .build();

        let condition = expression
            .condition_expression
            .as_deref()
            .expect("A condition expression should be set");
        assert_ne!("", condition);
        println!("{condition}");

        println!("Names: {:?}", expression.expression_attribute_names);
        println!("Values: {:?}", expression.expression_attribute_values);

        let update = expression.to_update_builder().build();

        println!("{update:#?}");
    }
}
