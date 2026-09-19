use serde::{Deserialize, Serialize};

use crate::keys::Key;

/// A source identifier.
#[derive(Serialize, Deserialize)]
pub struct SourceKey(String);

impl Key for SourceKey {}

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