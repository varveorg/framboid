pub mod working;

use blake3::Hash;
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