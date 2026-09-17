use crate::account::profile::period::Period;

/// A way a `Name` is or was used.
#[derive(Clone, Copy)]
pub struct Usage(UsageKind);

/// The kind of a `Usage`.
#[derive(Clone, Copy)]
pub enum UsageKind {
    Legal,
    Prior(Period),
    Used
}

impl Usage {
    /// Creates a `Usage` with a legal `UsageKind`.
    pub fn legal() -> Self {
        Self(UsageKind::Legal)
    }

    /// Creates a `Usage` with a prior `UsageKind`.
    pub fn prior(period: Period) -> Self {
        Self(UsageKind::Prior(period))
    }

    /// Creates a `Usage` with a used `UsageKind`.
    pub fn used() -> Self {
        Self(UsageKind::Used)
    }

    /// Returns a copy of the contained `UsageKind`.
    pub fn kind(&self) -> UsageKind {
        self.0
    }
}