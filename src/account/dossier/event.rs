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

pub struct EventError;

impl Event {
    /// Creates an `Event` by sorting by `Timestamp`, hashing every `Action` to create its key, and then inserts them into the map.
    pub fn new(mut actions: Vec<Action>) -> Result<Self, EventError> {
        if actions.is_empty() {
            return Err(EventError);
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
}