use blake3::Hash;
use indexmap::IndexMap;
use time::Timestamp;

use crate::account::dossier::{Dossier, event::Event};

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

    pub fn complete(self) -> Dossier {
        Dossier {
            events: self.events,
            from: self.from,
            to: Timestamp::now()
        }
    }
}