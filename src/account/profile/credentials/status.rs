/// An education's status.
#[derive(Clone, Copy)]
pub struct Status(StatusKind);

/// The kind of a `Status`.
#[derive(Clone, Copy)]
pub enum StatusKind {
    Completed,
    Enrolled,
    DroppedOut,
    Transferred,
    Withdrawn
}

impl Status {
    /// Creates a `Status` with a completed `StatusKind`.
    pub fn completed() -> Self {
        Self(StatusKind::Completed)
    }
    
    /// Creates a `Status` with an enrolled `StatusKind`.
    pub fn enrolled() -> Self {
        Self(StatusKind::Enrolled)
    }

    /// Creates a `Status` with a dropped-out `StatusKind`.
    pub fn dropped_out() -> Self {
        Self(StatusKind::DroppedOut)
    }

    /// Creates a `Status` with a transferred `StatusKind`.
    pub fn transferred() -> Self {
        Self(StatusKind::Transferred)
    }

    /// Creates a `Status` with a withdrawn `StatusKind`.
    pub fn withdrawn() -> Self {
        Self(StatusKind::Withdrawn)
    }

    /// Returns a copy of the contained `StatusKind`.
    pub fn kind(&self) -> StatusKind {
        self.0
    }
}