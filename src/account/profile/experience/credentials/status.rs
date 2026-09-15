pub struct Status (StatusKind);
enum StatusKind {
    Completed,
    Enrolled,
    DroppedOut,
    Transferred,
    Withdrawn
}

impl Status {
    pub fn completed() -> Self {
        Self(StatusKind::Completed)
    }
    
    pub fn enrolled() -> Self {
        Self(StatusKind::Enrolled)
    }

    pub fn dropped_out() -> Self {
        Self(StatusKind::DroppedOut)
    }

    pub fn transferred() -> Self {
        Self(StatusKind::Transferred)
    }

    pub fn withdrawn() -> Self {
        Self(StatusKind::Withdrawn)
    }
}