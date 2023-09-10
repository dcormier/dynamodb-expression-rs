mod to_builders;

use alloc::borrow::Cow;

use aws_sdk_dynamodb::types::AttributeValue;
use itermap::IterMap;
use optempty::EmptyIntoNone;
use std::collections::HashMap;

use crate::{
    condition::{
        And, AttributeExists, AttributeNotExists, AttributeType, BeginsWith, Between, Comparison,
        Condition, Contains, In, Not, Or, Parenthetical,
    },
    key::KeyCondition,
    operand::{Operand, Size},
    value::{scalar::ScalarType, Value, ValueType},
    Name, ScalarValue,
};

#[derive(Debug, Clone)]
pub struct Expression {
    condition: Option<Condition>,
    key_condition: Option<KeyCondition>,
    filter: Option<Condition>,
    projection: Option<Vec<Name>>,
    names: HashMap<Cow<'static, str>, Cow<'static, str>>,
    values: HashMap<ValueType, Cow<'static, str>>,
}

/// For building an expression.
impl Expression {
    // Intentionally private.
    fn new() -> Self {
        Self {
            condition: None,
            key_condition: None,
            filter: None,
            projection: None,
            names: HashMap::default(),
            values: HashMap::default(),
        }
    }

    /// Created a new `Expression` from to be used as a condition for DynamoDB input.
    pub fn new_with_condition<T>(condition: T) -> Self
    where
        T: Into<Condition>,
    {
        Self::new().with_condition(condition.into())
    }

    /// Sets the condition for this expression, overwriting any previously set.
    pub fn with_condition<T>(mut self, condition: T) -> Self
    where
        T: Into<Condition>,
    {
        self.condition = Some(self.process_condition(condition.into()));

        self
    }

    /// Created a new `Expression` from to be used as a key_condition for DynamoDB input.
    pub fn new_with_key_condition<T>(key_condition: T) -> Self
    where
        T: Into<KeyCondition>,
    {
        Self::new().with_key_condition(key_condition.into())
    }

    /// Sets the key_condition for this expression, overwriting any previously set.
    pub fn with_key_condition<T>(mut self, key_condition: T) -> Self
    where
        T: Into<KeyCondition>,
    {
        self.key_condition = Some(KeyCondition {
            condition: self.process_condition(key_condition.into().condition),
        });

        self
    }

    /// Created a new `Expression` from to be used as a filter for DynamoDB input.
    pub fn new_with_filter<T>(filter: T) -> Self
    where
        T: Into<Condition>,
    {
        Self::new().with_filter(filter.into())
    }

    /// Sets the filter for this expression, overwriting any previously set.
    pub fn with_filter<T>(mut self, filter: T) -> Self
    where
        T: Into<Condition>,
    {
        self.filter = Some(self.process_condition(filter.into()));

        self
    }

    pub fn new_with_projection<I, T>(names: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Name>,
    {
        Self::new().with_projection(names)
    }

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
                operand: self.process_value(ValueType::from(operand)),
            }
            .into(),
            Condition::BeginsWith(BeginsWith { path, substr }) => BeginsWith {
                path: self.process_name(path),
                substr: self.process_value(ValueType::from(substr)),
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
        match operand {
            Operand::Name(name) => self.process_name(name).into(),
            Operand::Size(Size { name }) => Size {
                name: self.process_name(name),
            }
            .into(),
            Operand::Value(value) => self.process_value(ValueType::from(value)).into(),
            Operand::Condition(condition) => self.process_condition(*condition).into(),
        }
    }

    fn process_name(&mut self, name: Name) -> Name {
        let count = self.names.len();

        Name {
            name: self
                .names
                .entry(name.name.clone())
                .or_insert(format!("#{}", count).into())
                .clone(),
        }
    }

    fn process_value<T>(&mut self, value: T) -> ScalarValue
    where
        T: Into<Value>,
    {
        let count = self.values.len();

        ScalarValue {
            value: ScalarType::String(
                self.values
                    .entry(value.into().value)
                    .or_insert(format!(":{}", count).into())
                    .clone(),
            ),
        }
    }
}

/// Methods to get the values needed for DynamoDB input builders.
impl Expression {
    /// The string to use as a DynamoDB condition.
    pub fn condition_expression(&self) -> String {
        self.condition
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default()
    }

    /// For use on a key condition expression.
    pub fn key_condition_expression(&self) -> String {
        self.key_condition
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default()
    }

    /// The string to use as a DynamoDB filter.
    pub fn filter_expression(&self) -> String {
        self.filter
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default()
    }

    /// Gets the projection expression to use for a DynamoDB input.
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

    /// The expression attribute names for the DynamoDB input.
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

    /// The expression attribute values for the DynamoDB input.
    pub fn attribute_values(&self) -> Option<HashMap<String, AttributeValue>> {
        Some(
            self.values
                .iter()
                .map_values(ToString::to_string)
                .map_keys(|k| k.clone().into())
                .map(|(k, v)| (v, k))
                .collect(),
        )
        .empty_into_none()
    }
}
