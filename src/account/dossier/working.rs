use bitcode::serialize;
use blake3::{Hash, hash};
use indexmap::IndexMap;
use time::Timestamp;

use crate::account::{dossier::{Dossier, event::Event}, errors::{VarveError, dossier::DossierError}};

/// A `Dossier` that can be created and modified before completion.
pub struct WorkingDossier (IndexMap<Hash, Event>);

impl WorkingDossier {
    /// Constructs a `WorkingDossier` with supplied events.
    pub fn new(mut events: Vec<Event>) -> Self {
        events.sort_by_key(|event| event.from);
        
        let mut map = IndexMap::new();
        for event in events {
            map.insert_sorted_by_key(hash(&serialize(&event).unwrap()), event, |_, event| event.from);
        }

        Self (map)
    }

    /// Adds to the `WorkingDossier` with supplied events.
    pub fn add(&mut self, events: Vec<Event>) {
        for event in events {
            self.0.insert_sorted_by_key(hash(&serialize(&event).unwrap()), event, |_, event| event.from);
        }
    }

    /// Removes from the `WorkingDossier` with the `Hash` for the supplied `Event`.
    pub fn remove(&mut self, hash: Hash) -> Result<Event, VarveError> {
        if let Some(event) = self.0.shift_remove(&hash) {
            Ok(event)
        }
        else {
            Err(VarveError::DossierError(DossierError::IncorrectHash))
        }
    }

    /// Completes this `WorkingDossier` by setting the start and end timestamps.
    pub fn complete(self, from: Timestamp, to: Timestamp) -> Result<Dossier, VarveError> {
        if self.0.is_empty() {
            return Err(VarveError::DossierError(DossierError::EmptyDossier));
        }
        
        Ok(
            Dossier {
                events: self.0,
                from,
                to
            }
        )
    }
}