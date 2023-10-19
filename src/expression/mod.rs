mod to_aws;

use std::collections::HashMap;

use aws_sdk_dynamodb::types::AttributeValue;
use itermap::IterMap;
use optempty::EmptyIntoNone;

use crate::{
    condition::{
        And, AttributeExists, AttributeNotExists, AttributeType, BeginsWith, Between, Comparison,
        Condition, Contains, In, Not, Or, Parenthetical,
    },
    key::KeyCondition,
    name::Name,
    operand::{Operand, OperandType, Size},
    update::Update,
    value::{Ref, Value, ValueOrRef},
};

#[derive(Debug, Default, Clone)]
pub struct Expression {
    condition: Option<Condition>,
    key_condition: Option<KeyCondition>,
    update: Option<Update>,
    filter: Option<Condition>,
    projection: Option<Vec<Name>>,
    names: HashMap<Name, String>,
    values: HashMap<Value, Ref>,
}

// Functions and methods for building an expression.
impl Expression {
    /// Creates a new [`Expression`] with the specified condition for DynamoDB input.
    pub fn new_with_condition<T>(condition: T) -> Self
    where
        T: Into<Condition>,
    {
        Self::default().with_condition(condition.into())
    }

    /// Sets the condition for this [`Expression`], overwriting any previously set.
    pub fn with_condition<T>(mut self, condition: T) -> Self
    where
        T: Into<Condition>,
    {
        self.condition = Some(self.process_condition(condition.into()));

        self
    }

    /// Creates a new [`Expression`] with the specified key condition for DynamoDB input.
    pub fn new_with_key_condition<T>(key_condition: T) -> Self
    where
        T: Into<KeyCondition>,
    {
        Self::default().with_key_condition(key_condition.into())
    }

    /// Sets the key condition for this [`Expression`], overwriting any previously set.
    pub fn with_key_condition<T>(mut self, key_condition: T) -> Self
    where
        T: Into<KeyCondition>,
    {
        self.key_condition = Some(KeyCondition {
            condition: self.process_condition(key_condition.into().condition),
        });

        self
    }

    /// Creates a new [`Expression`] with the specified update for DynamoDB input.
    pub fn new_with_update<T>(update: T) -> Self
    where
        T: Into<Update>,
    {
        Self::default().with_update(update)
    }

    /// Sets the update expression, overwriting any previously set.
    pub fn with_update<T>(mut self, update: T) -> Self
    where
        T: Into<Update>,
    {
        self.update = Some(update.into());

        self
    }

    /// Creates a new [`Expression`] with the specified filter for DynamoDB input.
    pub fn new_with_filter<T>(filter: T) -> Self
    where
        T: Into<Condition>,
    {
        Self::default().with_filter(filter.into())
    }

    /// Sets the filter for this [`Expression`], overwriting any previously set.
    pub fn with_filter<T>(mut self, filter: T) -> Self
    where
        T: Into<Condition>,
    {
        self.filter = Some(self.process_condition(filter.into()));

        self
    }

    /// Creates a new [`Expression`] with the specified projection for DynamoDB input.
    pub fn new_with_projection<I, T>(names: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Name>,
    {
        Self::default().with_projection(names)
    }

    /// Sets the projection for this [`Expression`], overwriting any previously set.
    pub fn with_projection<I, T>(mut self, names: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Name>,
    {
        self.projection = Some(
            names
                .into_iter()
                .map(|name| self.process_name(name.into()))
                .collect(),
        );

        self
    }

    fn process_condition(&mut self, condition: Condition) -> Condition {
        match condition {
            Condition::AttributeExists(AttributeExists { name }) => AttributeExists {
                name: self.process_name(name),
            }
            .into(),
            Condition::AttributeNotExists(AttributeNotExists { name }) => AttributeNotExists {
                name: self.process_name(name),
            }
            .into(),
            Condition::AttributeType(AttributeType {
                path,
                attribute_type,
            }) => AttributeType {
                path: self.process_name(path),
                attribute_type,
            }
            .into(),
            Condition::Contains(Contains { path, operand }) => Contains {
                path: self.process_name(path),
                operand: self.process_value(operand).into(),
            }
            .into(),
            Condition::BeginsWith(BeginsWith { path, substr }) => BeginsWith {
                path: self.process_name(path),
                substr: self.process_value(substr).into(),
            }
            .into(),
            Condition::Between(Between { op, lower, upper }) => Between {
                op: self.process_operand(op),
                lower: self.process_operand(lower),
                upper: self.process_operand(upper),
            }
            .into(),
            Condition::In(In { op, items }) => In {
                op: self.process_operand(op),
                items: items
                    .into_iter()
                    .map(|item| self.process_operand(item))
                    .collect(),
            }
            .into(),
            Condition::Comparison(Comparison { left, cmp, right }) => Comparison {
                left: self.process_operand(left),
                cmp,
                right: self.process_operand(right),
            }
            .into(),
            Condition::And(And { left, right }) => And {
                left: self.process_condition(*left).into(),
                right: self.process_condition(*right).into(),
            }
            .into(),
            Condition::Or(Or { left, right }) => Or {
                left: self.process_condition(*left).into(),
                right: self.process_condition(*right).into(),
            }
            .into(),
            Condition::Not(Not { condition }) => Not {
                condition: self.process_condition(*condition).into(),
            }
            .into(),
            Condition::Parenthetical(Parenthetical { condition }) => Parenthetical {
                condition: self.process_condition(*condition).into(),
            }
            .into(),
        }
    }

    fn process_operand(&mut self, operand: Operand) -> Operand {
        match operand.op {
            OperandType::Name(name) => self.process_name(name).into(),
            OperandType::Size(Size { name }) => Size {
                name: self.process_name(name),
            }
            .into(),
            OperandType::Value(value) => Operand {
                op: OperandType::Value(self.process_value(value).into()),
            },
            OperandType::Condition(condition) => self.process_condition(*condition).into(),
        }
    }

    fn process_name(&mut self, name: Name) -> Name {
        let count = self.names.len();

        Name {
            name: self
                .names
                .entry(name)
                .or_insert(format!("#{}", count))
                .clone(),
        }
    }

    fn process_value(&mut self, value: ValueOrRef) -> Ref {
        match value {
            ValueOrRef::Value(value) => {
                let count = self.values.len();

                self.values
                    .entry(value)
                    .or_insert_with(|| count.to_string().into())
                    .clone()
            }
            ValueOrRef::Ref(value) => value,
        }
    }
}

// Methods to get the values needed for DynamoDB input builders.
impl Expression {
    /// The string to use as a DynamoDB condition expression. Be sure to also use
    /// `.attribute_names()` and `.attribute_values()`.
    pub fn condition_expression(&self) -> String {
        self.condition
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default()
    }

    /// The string to use use as a DynamoDB key condition expression. Be sure to
    /// also use `.attribute_names()` and `.attribute_values()`.
    pub fn key_condition_expression(&self) -> String {
        self.key_condition
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default()
    }

    /// The string to use as a DynamoDB filter expression. Be sure to also use
    /// `.attribute_names()` and `.attribute_values()`.
    pub fn filter_expression(&self) -> String {
        self.filter
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default()
    }

    /// The string to use as a DynamoDB projection expression. Be sure to also
    /// use `.attribute_names()` and `.attribute_values()`.
    ///
    /// See: <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ProjectionExpressions.html>
    pub fn projection_expression(&self) -> String {
        self.projection
            .as_ref()
            .into_iter()
            .flat_map(|projection| projection.iter())
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// DynamoDB expression attribute names.
    pub fn attribute_names(&self) -> Option<HashMap<String, String>> {
        Some(
            self.names
                .iter()
                .map_keys(ToString::to_string)
                .map_values(ToString::to_string)
                .map(|(k, v)| (v, k))
                .collect(),
        )
        .empty_into_none()
    }

    /// DynamoDB expression attribute values.
    pub fn attribute_values(&self) -> Option<HashMap<String, AttributeValue>> {
        Some(
            self.values
                .iter()
                .map_values(ToString::to_string)
                .map_keys(|k| k.clone().into_attribute_value())
                .map(|(k, v)| (v, k))
                .collect(),
        )
        .empty_into_none()
    }
}
