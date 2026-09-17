pub mod operations;

use bitcode::serialize;
use blake3::{Hash, hash};
use indexmap::IndexMap;

use crate::addressing::Action;

/// An `Event` that can be created and modified before completion.
pub struct WorkingEvent (IndexMap<Hash, Action>);

impl WorkingEvent {
    /// Creates a `WorkingEvent` with supplied actions.
    pub fn new(mut actions: Vec<Action>) -> Self {
        actions.sort_by_key(|action| action.time);
        
        let mut map = IndexMap::new();
        for action in actions {
            map.insert(hash(&serialize(&action).unwrap()), action);
        }

        Self (map)
    }

    /// Returns a reference to the contained actions.
    pub fn actions(&self) -> &IndexMap<Hash, Action> {
        &self.0
    }
}