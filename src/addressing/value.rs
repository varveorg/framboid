use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

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