pub mod deposit;
pub mod excavate;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use time::Timestamp;

/// A single, discrete action.
#[derive(Serialize, Deserialize)]
pub struct Action {
    name: String,
    source: String,
    pub(super) time: Timestamp,
    body: Value
}

/// A set of types for polymorphism.
#[derive(Serialize, Deserialize)]
pub enum Value {
    Bool(bool),
    Handle(String),
    Integer(i64),
    List(Vec<Value>),
    Map(IndexMap<String, Value>),
    Text(String)
}

impl Action {
    /// Creates an `Action`.
    pub fn new(name: String, source: String, time: Timestamp, body: Value) -> Self {
        Self {
            name,
            source,
            time,
            body
        }
    }
}