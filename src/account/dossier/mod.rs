pub mod event;
pub mod working;

use blake3::Hash;
use indexmap::IndexMap;
use time::Timestamp;

use crate::account::{claim::Claim, dossier::event::Event};

pub struct Dossier {
    claims: IndexMap<Hash, Claim>,
    events: IndexMap<Hash, Event>,
    from: Timestamp,
    to: Timestamp
}