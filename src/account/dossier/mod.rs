pub mod event;
pub mod working;

use blake3::Hash;
use indexmap::IndexMap;
use serde::Serialize;
use time::Timestamp;

use crate::account::dossier::event::Event;

/// A bounded span of events with a coherent identity.
#[derive(Serialize)]
pub struct Dossier {
    pub(super) events: IndexMap<Hash, Event>,
    pub(super) from: Timestamp,
    to: Timestamp
}