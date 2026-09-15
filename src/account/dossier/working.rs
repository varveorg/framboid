use bitcode::serialize;
use blake3::{Hash, hash};
use indexmap::IndexMap;
use time::Timestamp;

use crate::account::dossier::{Dossier, event::Event};

/// A `Dossier` that can be created and modified before completion.
pub struct WorkingDossier {
    events: IndexMap<Hash, Event>,
    from: Timestamp
}

impl WorkingDossier {
    /// Constructs a `WorkingDossier`.
    pub fn new() -> WorkingDossier {
        WorkingDossier {
            events: IndexMap::new(),
            from: Timestamp::now(),
        }
    }

    pub fn add_events(&mut self, events: Vec<Event>) {
        for event in events {
            self.events.insert_sorted_by_key(hash(&serialize(&event).unwrap()), event, |_, event| event.from);
        }
    }

    /// Completes this `WorkingDossier` by setting its end time to the current time.
    pub fn complete(self) -> Dossier {
        Dossier {
            events: self.events,
            from: self.from,
            to: Timestamp::now()
        }
    }
}