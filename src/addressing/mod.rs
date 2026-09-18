pub mod deposit;
pub mod excavate;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use time::Timestamp;

use crate::keys::source::SourceKey;

/// A single, discrete action.
#[derive(Serialize, Deserialize)]
pub struct Action {
    name: String,
    source: SourceKey,
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
    pub fn new(
        name: String,
        source: SourceKey,
        time: Timestamp,
        body: Value
    ) -> Self {
        Self {
            name,
            source,
            time,
            body
        }
    }

    /// Returns a reference to the contained name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a reference to the contained `SourceKey`.
    pub fn source(&self) -> &SourceKey {
        &self.source
    }

    /// Returns a copy of the contained `Timestamp`.
    pub fn time(&self) -> Timestamp {
        self.time
    }

    /// Returns a reference to the contained `Value`.
    pub fn body(&self) -> &Value {
        &self.body
    }
}