pub mod event;
pub mod working;

use blake3::Hash;
use indexmap::IndexMap;
use time::Timestamp;

use crate::account::dossier::event::Event;

/// A bounded span of events with a coherent identity.
pub struct Dossier {
    events: IndexMap<Hash, Event>,
    from: Timestamp,
    to: Timestamp
}