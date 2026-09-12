use blake3::Hash;
use indexmap::IndexMap;
use time::Timestamp;

use crate::account::dossier::{Dossier, event::Event};

/// A `Dossier` that can be created and modified before completion.
pub struct WorkingDossier {
    events: IndexMap<Hash, Event>,
    from: Timestamp
}

impl WorkingDossier {
    pub fn new() -> WorkingDossier {
        WorkingDossier {
            events: IndexMap::new(),
            from: Timestamp::now(),
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