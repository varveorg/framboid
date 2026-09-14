use blake3::Hash;
use indexmap::IndexMap;
use time::Timestamp;

use crate::addressing::Action;

/// A slice of time deemed meaningful.
pub struct Event {
    actions: IndexMap<Hash, Action>,
    from: Timestamp,
    to: Timestamp
}