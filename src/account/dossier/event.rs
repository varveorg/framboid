use time::Timestamp;

/// A slice of time deemed meaningful.
pub struct Event {
    from: Timestamp,
    to: Timestamp
}