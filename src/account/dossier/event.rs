use std::collections::HashSet;

use time::Timestamp;

pub struct Event {
    kind: EventKind,
    // Decide type soon
    instruments: HashSet<()>,
    // Decide type soon
    mediums: HashSet<()>,
    from: Timestamp,
    to: Timestamp
}

pub enum EventKind {
    Natural,
    Synthetic
}