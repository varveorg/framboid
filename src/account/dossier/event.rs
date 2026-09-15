use bitcode::serialize;
use blake3::{Hash, hash};
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

pub enum EventError {
    EmptyActions,
    IncorrectHash
}

impl Event {
    /// Creates an `Event` by sorting by `Timestamp`, hashing every `Action` to create its key, and then inserts them into the map.
    pub fn new(mut actions: Vec<Action>) -> Result<Self, EventError> {
        if actions.is_empty() {
            return Err(EventError::EmptyActions);
        }

        actions.sort_by_key(|action| action.time);
        
        let mut map = IndexMap::new();
        for action in actions {
            map.insert(hash(&serialize(&action).unwrap()), action);
        }

        let actions = map;
        let from = actions.first().unwrap().1.time;
        let to = actions.last().unwrap().1.time;

        Ok(
            Self {
                actions,
                from,
                to
            }
        )
    }

    /// Adds an `Action` to the map while preserving insertion order.
    pub fn add(&mut self, action: Action) {
        self.actions.insert_sorted_by_key(hash(&serialize(&action).unwrap()), action, |_, action| action.time);
    }

    /// Removes an `Action` while preserving insertion order.
    pub fn remove(&mut self, hash: Hash) -> Result<Action, EventError> {
        if let Some(action) = self.actions.shift_remove(&hash) {
            Ok(action)
        }
        else {
            Err(EventError::IncorrectHash)
        }
    }
}