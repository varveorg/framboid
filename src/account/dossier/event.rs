use std::collections::HashSet;

use time::Timestamp;

use crate::descriptors::{instruments::Instrument, mediums::Medium};

pub struct Event {
    kind: EventKind,
    instruments: HashSet<Instrument>,
    mediums: HashSet<Medium>,
    from: Timestamp,
    to: Timestamp
}

pub enum EventKind {
    Natural,
    Synthetic
}