use serde::{Deserialize, Serialize};

/// A source identifier.
#[derive(Serialize, Deserialize)]
pub struct SourceKey(String);

impl SourceKey {
    /// Creates a `SourceKey`.
    pub fn new(key: String) -> Self {
        Self(key)
    }

    /// Returns a reference to the contained key.
    pub fn key(&self) -> &str {
        &self.0
    }
}