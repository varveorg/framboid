use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// A set of types for polymorphism.
#[derive(Serialize, Deserialize)]
pub struct Body(BodyKind);

/// The kind of a `Body`.
#[derive(Serialize, Deserialize)]
pub enum BodyKind {
    Boolean(bool),
    Handle(String),
    Integer(i64),
    List(Vec<Body>),
    Map(IndexMap<String, Body>),
    Text(String)
}

impl Body {
    /// Creates a `Body` with a boolean `BodyKind`.
    pub fn bool(boolean: bool) -> Self {
        Self(BodyKind::Boolean(boolean))
    }

    /// Creates a `Body` with a handle `BodyKind`.
    pub fn handle(handle: String) -> Self {
        Self(BodyKind::Handle(handle))
    }

    /// Creates a `Body` with an integer `BodyKind`.
    pub fn integer(integer: i64) -> Self {
        Self(BodyKind::Integer(integer))
    }

    /// Creates a `Body` with a list `BodyKind`.
    pub fn list(list: Vec<Self>) -> Self {
        Self(BodyKind::List(list))
    }

    /// Creates a `Body` with a map `BodyKind`.
    pub fn map(map: IndexMap<String, Self>) -> Self {
        Self(BodyKind::Map(map))
    }

    /// Creates a `Body` with a text `BodyKind`.
    pub fn text(text: String) -> Self {
        Self(BodyKind::Text(text))
    }

    /// Returns a reference to the contained `BodyKind`.
    pub fn kind(&self) -> &BodyKind {
        &self.0
    }
}