use indexmap::IndexMap;

/// A set of types for polymorphism.
pub enum Value {
    Bool(bool),
    Handle(String),
    Integer(i64),
    List(Vec<Value>),
    Map(IndexMap<String, Value>),
    Text(String)
}