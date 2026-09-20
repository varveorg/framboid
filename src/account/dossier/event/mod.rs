pub mod working;

use blake3::Hash;
use indexmap::IndexMap;
use serde::Serialize;
use time::Timestamp;

use crate::addressing::Action;

/// A slice of time deemed meaningful.
#[derive(Serialize)]
pub struct Event {
    actions: IndexMap<Hash, Action>,
    pub(super) from: Timestamp,
    to: Timestamp
}

impl Event {
    /// Returns a reference to the contained actions.
    pub fn actions(&self) -> &IndexMap<Hash, Action> {
        &self.actions
    }

    /// Returns a copy of the contained from `Timestamp`.
    pub fn from(&self) -> Timestamp {
        self.from
    }

    /// Returns a copy of the contained to `Timestamp`.
    pub fn to(&self) -> Timestamp {
        self.to
    }
}