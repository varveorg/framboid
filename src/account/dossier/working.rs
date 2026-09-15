use bitcode::serialize;
use blake3::{Hash, hash};
use indexmap::IndexMap;
use time::Timestamp;

use crate::account::dossier::{Dossier, event::Event};

/// A `Dossier` that can be created and modified before completion.
pub struct WorkingDossier (IndexMap<Hash, Event>);

impl WorkingDossier {
    /// Constructs a `WorkingDossier`.
    pub fn new() -> WorkingDossier {
        WorkingDossier (IndexMap::new())
    }

    pub fn add_events(&mut self, events: Vec<Event>) {
        for event in events {
            self.0.insert_sorted_by_key(hash(&serialize(&event).unwrap()), event, |_, event| event.from);
        }
    }

    /// Completes this `WorkingDossier` by setting the start and end timestamps.
    pub fn complete(self, from: Timestamp, to: Timestamp) -> Dossier {
        Dossier {
            events: self.0,
            from,
            to
        }
    }
}