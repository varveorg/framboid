pub mod body;
pub mod deposit;
pub mod excavate;

use serde::{Deserialize, Serialize};
use time::Timestamp;

use crate::{addressing::body::Body, keys::source::SourceKey};

/// A single, discrete action.
#[derive(Serialize, Deserialize)]
pub struct Action {
    name: String,
    source: SourceKey,
    pub(super) time: Timestamp,
    body: Body
}



impl Action {
    /// Creates an `Action`.
    pub fn new(
        name: String,
        source: SourceKey,
        time: Timestamp,
        body: Body
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

    /// Returns a reference to the contained `Body`.
    pub fn body(&self) -> &Body {
        &self.body
    }
}