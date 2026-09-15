use crate::account::profile::experience::location::Location;

pub struct Education {
    pub(super) degree: String,
    pub(super) school: String,
    pub(super) status: Status,
    pub(super) location: Location,
    pub(super) discipline: String
}

pub struct Status(StatusKind);
enum StatusKind {
    Enrolled,
    DroppedOut,
    Transferred,
    Withdrawn
}

impl Status {
    pub fn enrolled() -> Self {
        Status(StatusKind::Enrolled)
    }

    pub fn dropped_out() -> Self {
        Status(StatusKind::DroppedOut)
    }

    pub fn transferred() -> Self {
        Status(StatusKind::Transferred)
    }

    pub fn withdrawn() -> Self {
        Status(StatusKind::Withdrawn)
    }
}