use std::collections::HashMap;

use itertools::Itertools;
use optempty::EmptyIntoNone;

use super::Expression;
use crate::{
    condition::{
        And, AttributeExists, AttributeNotExists, AttributeType, BeginsWith, Between, Comparison,
        Condition, Contains, In, Not, Or, Parenthetical,
    },
    key::KeyCondition,
    operand::{Operand, OperandType, Size},
    path::{Element, Name, Path},
    update::{set::SetAction, Update},
    value::{Ref, Value, ValueOrRef},
};

#[must_use = "Call `.build()` to create the `Expression`"]
#[derive(Debug, Default, Clone)]
pub struct Builder {
    condition: Option<Condition>,
    key_condition: Option<KeyCondition>,
    update: Option<Update>,
    filter: Option<Condition>,
    projection: Option<Vec<Name>>,
    names: HashMap<Name, String>,
    values: HashMap<Value, Ref>,
}

/// Functions and methods for building an `Expression`.
impl Builder {
    /// Sets the condition for this [`Expression`], overwriting any previously set.
    pub fn with_condition<T>(mut self, condition: T) -> Self
    where
        T: Into<Condition>,
    {
        self.condition = Some(self.process_condition(condition.into()));

        self
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

    /// Sets the update expression, overwriting any previously set.
    pub fn with_update<T>(mut self, update: T) -> Self
    where
        T: Into<Update>,
    {
        self.update = Some(self.process_update(update.into()));

        self
    }

    /// Sets the filter for this [`Expression`], overwriting any previously set.
    pub fn with_filter<T>(mut self, filter: T) -> Self
    where
        T: Into<Condition>,
    {
        self.filter = Some(self.process_condition(filter.into()));

        self
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
        )
        // Empty into `None` because DynamoDB doesn't allow empty projection expressions.
        // `Invalid ProjectionExpression: The expression can not be empty;`
        .empty_into_none();

        self
    }

    /// Builds the [`Expression`].
    pub fn build(self) -> Expression {
        let Self {
            condition,
            key_condition,
            update,
            filter,
            projection,
            names,
            values,
        } = self;

        Expression {
            condition_expression: condition.map(Into::into),
            key_condition_expression: key_condition.map(Into::into),
            update_expression: {
                // Is there a more efficient way when all the `Update` strings
                // require formatting?
                update.as_ref().map(ToString::to_string)
            },
            filter_expression: filter.map(Into::into),
            projection_expression: projection.map(|attrs| {
                attrs
                    .into_iter()
                    .map(|name| name.name)
                    .collect_vec()
                    .join(", ")
            }),
            expression_attribute_names: Some(
                names
                    .into_iter()
                    .map(|(name, name_ref)| (name_ref, name.name))
                    .collect(),
            )
            .empty_into_none(),
            expression_attribute_values: Some(
                values
                    .into_iter()
                    .map(|(value, value_ref)| (value_ref.into(), value.into_attribute_value()))
                    .collect(),
            )
            .empty_into_none(),
        }
    }

    fn process_condition(&mut self, condition: Condition) -> Condition {
        match condition {
            Condition::AttributeExists(AttributeExists { path }) => AttributeExists {
                path: self.process_path(path),
            }
            .into(),
            Condition::AttributeNotExists(AttributeNotExists { path }) => AttributeNotExists {
                path: self.process_path(path),
            }
            .into(),
            Condition::AttributeType(AttributeType {
                path,
                attribute_type,
            }) => AttributeType {
                path: self.process_path(path),
                attribute_type,
            }
            .into(),
            Condition::Contains(Contains { path, operand }) => Contains {
                path: self.process_path(path),
                operand: self.process_value(operand).into(),
            }
            .into(),
            Condition::BeginsWith(BeginsWith { path, substr }) => BeginsWith {
                path: self.process_path(path),
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
            OperandType::Path(path) => self.process_path(path).into(),
            OperandType::Size(Size { path: name }) => Size {
                path: self.process_path(name),
            }
            .into(),
            OperandType::Scalar(value) => Operand {
                op: OperandType::Scalar(self.process_value(value).into()),
            },
            OperandType::Condition(condition) => self.process_condition(*condition).into(),
        }
    }

    fn process_update(&mut self, update: Update) -> Update {
        match update {
            Update::Set(mut update) => {
                update.actions = update
                    .actions
                    .into_iter()
                    .map(|action| match action {
                        SetAction::Assign(mut action) => {
                            action.path = self.process_path(action.path);
                            action.value = self.process_value(action.value).into();

                            action.into()
                        }
                        SetAction::Math(mut action) => {
                            action.dst = self.process_path(action.dst);
                            action.src = action.src.map(|src| self.process_path(src));
                            action.num = self.process_value(action.num).into();

                            action.into()
                        }
                        SetAction::ListAppend(mut action) => {
                            action.dst = self.process_path(action.dst);
                            action.src = action.src.map(|src| self.process_path(src));
                            action.list = self.process_value(action.list).into();

                            action.into()
                        }
                        SetAction::IfNotExists(mut action) => {
                            action.dst = self.process_path(action.dst);
                            action.src = action.src.map(|src| self.process_path(src));
                            action.value = self.process_value(action.value).into();

                            action.into()
                        }
                    })
                    .collect();

                update.into()
            }
            Update::Remove(mut update) => {
                update.paths = update
                    .paths
                    .into_iter()
                    .map(|path| self.process_path(path))
                    .collect();

                update.into()
            }
            Update::Add(mut update) => {
                update.path = self.process_path(update.path);
                update.value = self.process_value(update.value).into();

                update.into()
            }
            Update::Delete(mut update) => {
                update.path = self.process_path(update.path);
                update.subset = self.process_value(update.subset).into();

                update.into()
            }
        }
    }

    fn process_path(&mut self, mut path: Path) -> Path {
        path.elements = path
            .elements
            .into_iter()
            .map(|elem| match elem {
                Element::Name(name) => self.process_name(name).into(),
                Element::IndexedField(mut indexed_field) => {
                    indexed_field.name = self.process_name(indexed_field.name);

                    indexed_field.into()
                }
            })
            .collect();

        path
    }

    fn process_name(&mut self, name: Name) -> Name {
        let count = self.names.len();

        Name {
            name: self
                .names
                .entry(name)
                .or_insert(format!("#{count}"))
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
