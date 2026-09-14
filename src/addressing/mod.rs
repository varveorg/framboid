use indexmap::IndexMap;
use time::Timestamp;

/// A single, discrete action.
pub struct Action {
    name: String,
    source: String,
    time: Timestamp,
    body: Value
}

/// A set of types for polymorphism.
pub enum Value {
    Bool(bool),
    Handle(String),
    Integer(i64),
    List(Vec<Value>),
    Map(IndexMap<String, Value>),
    Text(String)
}
