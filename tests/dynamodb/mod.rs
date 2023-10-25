#![cfg(test)]

//! Helpers for running tests against DynamoDB.

pub mod config;
pub mod debug;
pub mod item;
pub mod partial_eq;
pub mod setup;

pub use self::{
    config::Config,
    debug::{DebugAttributeValue, DebugItem},
};
