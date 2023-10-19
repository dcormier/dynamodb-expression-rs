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
};

use super::Expression;

/// Conversions to DynamoDB input builders
impl Expression {
    pub fn to_put_builder(&self) -> PutBuilder {
        Put::builder()
            .condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    pub fn to_put_item_input_builder(&self) -> PutItemInputBuilder {
        PutItemInput::builder()
            .condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    pub fn to_put_item_fluent_builder(
        &self,
        builder: PutItemFluentBuilder,
    ) -> PutItemFluentBuilder {
        builder
            .condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    pub fn to_get_builder(&self) -> GetBuilder {
        Get::builder()
            .projection_expression(self.projection_expression())
            .set_expression_attribute_names(self.attribute_names())
    }

    pub fn to_get_item_input_builder(&self) -> GetItemInputBuilder {
        GetItemInput::builder()
            .projection_expression(self.projection_expression())
            .set_expression_attribute_names(self.attribute_names())
    }

    pub fn to_get_item_fluent_builder(
        &self,
        builder: GetItemFluentBuilder,
    ) -> GetItemFluentBuilder {
        builder
            .projection_expression(self.projection_expression())
            .set_expression_attribute_names(self.attribute_names())
    }

    pub fn to_update_builder(&self) -> UpdateBuilder {
        Update::builder()
            // TODO:
            // .update_expression(self.update_expression())
            .condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    pub fn to_update_item_input_builder(&self) -> UpdateItemInputBuilder {
        UpdateItemInput::builder()
            // TODO:
            // .update_expression(self.update_expression())
            .condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    pub fn to_update_item_fluent_builder(
        &self,
        builder: UpdateItemFluentBuilder,
    ) -> UpdateItemFluentBuilder {
        builder
            // TODO:
            // .update_expression(self.update_expression())
            .condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    pub fn to_delete_builder(&self) -> DeleteBuilder {
        Delete::builder()
            .condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    pub fn to_delete_item_input_builder(&self) -> DeleteItemInputBuilder {
        DeleteItemInput::builder()
            .condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    pub fn to_delete_item_fluent_builder(
        &self,
        builder: DeleteItemFluentBuilder,
    ) -> DeleteItemFluentBuilder {
        builder
            .condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    pub fn to_query_input_builder(&self) -> QueryInputBuilder {
        QueryInput::builder()
            .key_condition_expression(self.key_condition_expression())
            .filter_expression(self.filter_expression())
            .projection_expression(self.projection_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    pub fn to_query_fluent_builder(&self, builder: QueryFluentBuilder) -> QueryFluentBuilder {
        builder
            .key_condition_expression(self.key_condition_expression())
            .filter_expression(self.filter_expression())
            .projection_expression(self.projection_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    pub fn to_scan_input_builder(&self) -> ScanInputBuilder {
        ScanInput::builder()
            .projection_expression(self.projection_expression())
            .filter_expression(self.filter_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    pub fn to_scan_fluent_builder(&self, builder: ScanFluentBuilder) -> ScanFluentBuilder {
        builder
            .projection_expression(self.projection_expression())
            .filter_expression(self.filter_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }

    pub fn to_keys_and_attributes_builder(&self) -> KeysAndAttributesBuilder {
        KeysAndAttributes::builder()
            .projection_expression(self.projection_expression())
            .set_expression_attribute_names(self.attribute_names())
    }

    pub fn to_condition_check_builder(&self) -> ConditionCheckBuilder {
        ConditionCheck::builder()
            .condition_expression(self.condition_expression())
            .set_expression_attribute_names(self.attribute_names())
            .set_expression_attribute_values(self.attribute_values())
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::{assert_eq, assert_ne};

    use crate::{key::key, name, num_value, Comparator::*};

    use super::Expression;

    #[test]
    fn scan_input() {
        let expression = Expression::new_with_filter(
            name("name")
                .begins_with("prefix")
                .and(name("age").comparison(Ge, num_value(25))),
        )
        .with_projection(["name", "age"]);
        assert_eq!(None, expression.condition);

        let filter = expression.filter_expression();
        assert_ne!("", filter);
        println!("{filter}");

        let projection = expression.projection_expression();
        assert_ne!("", projection);
        println!("Projection: {projection}");

        println!("Names: {:?}", expression.names);
        println!("Values: {:?}", expression.values);

        let si = expression.to_scan_input_builder().build().unwrap();

        println!("{si:#?}");
    }

    #[test]
    fn query_input() {
        let expression = Expression::new_with_filter(
            name("name")
                .attribute_exists()
                .and(name("age").comparison(Ge, num_value(2.5))),
        )
        .with_projection(["name", "age"])
        .with_key_condition(key("id").equal(num_value(42)));
        assert_eq!(None, expression.condition);

        let filter = expression.filter_expression();
        assert_ne!("", filter);
        println!("{filter}");

        let projection = expression.projection_expression();
        assert_ne!("", projection);
        println!("Projection: {projection}");

        println!("Names: {:?}", expression.names);
        println!("Values: {:?}", expression.values);

        let qi = expression.to_query_input_builder().build().unwrap();

        println!("{qi:#?}");
    }

    #[test]
    fn update() {
        let expression = Expression::new_with_condition(
            name("name")
                .attribute_exists()
                .and(name("age").comparison(Ge, num_value(25))),
        );

        let condition = expression.condition_expression();
        assert_ne!("", condition);
        println!("{condition}");

        println!("Names: {:?}", expression.names);
        println!("Values: {:?}", expression.values);

        let update = expression.to_update_builder().build();

        println!("{update:#?}");
    }
}
