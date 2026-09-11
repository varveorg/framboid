use blake3::Hash;
use indexmap::IndexMap;
use time::Timestamp;

use crate::account::{claim::Claim, dossier::{Dossier, event::Event}};

pub struct WorkingDossier {
    claims: IndexMap<Hash, Claim>,
    events: IndexMap<Hash, Event>,
    from: Timestamp
}

impl WorkingDossier {
    pub fn new() -> WorkingDossier {
        WorkingDossier {
            claims: IndexMap::new(),
            events: IndexMap::new(),
            from: Timestamp::now(),
        }
    }

    pub fn complete(self) -> Dossier {
        Dossier {
            claims: self.claims,
            events: self.events,
            from: self.from,
            to: Timestamp::now()
        }
    }
}