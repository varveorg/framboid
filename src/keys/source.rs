use serde::{Deserialize, Serialize};

use crate::keys::Key;

/// A source identifier.
#[derive(Serialize, Deserialize)]
pub struct SourceKey(String);

impl Key for SourceKey {
    /// Creates a `SourceKey`.
    fn new(key: String) -> Self {
        Self(key)
    }

    /// Returns a reference to the contained key.
    fn key(&self) -> &str {
        &self.0
    }
}