pub mod event;
pub mod working;

use blake3::Hash;
use indexmap::IndexMap;
use serde::Serialize;
use time::Timestamp;

use crate::account::dossier::event::Event;

/// A bounded span of events with a coherent identity.
#[derive(Serialize)]
pub struct Dossier {
    pub(super) events: IndexMap<Hash, Event>,
    pub(super) from: Timestamp,
    to: Timestamp
}

impl Dossier {
    /// Returns a reference to the contained events.
    pub fn events(&self) -> &IndexMap<Hash, Event> {
        &self.events
    }

    /// Returns a copy of the from `Timestamp`.
    pub fn from(&self) -> Timestamp {
        self.from
    }

    /// Returns a copy of the to `Timestamp`.
    pub fn to(&self) -> Timestamp {
        self.to
    }
}