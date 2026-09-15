pub mod operations;

use bitcode::serialize;
use blake3::{Hash, hash};
use indexmap::IndexMap;

use crate::account::dossier::event::Event;

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

    pub fn events(&self) -> &IndexMap<Hash, Event> {
        &self.0
    }
}