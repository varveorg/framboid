/// A work arrangement for an `Engagement`.
#[derive(Clone, Copy)]
pub struct Arrangement(ArrangementKind);

/// The kind of an `Arrangement`.
#[derive(Clone, Copy)]
pub enum ArrangementKind {
    Hybrid,
    OnSite,
    Remote
}

impl Arrangement {
    /// Creates an `Arrangement` with a hybrid `ArrangementKind`.
    pub fn hybrid() -> Self {
        Self(ArrangementKind::Hybrid)
    }

    /// Creates an `Arrangement` with an on-site `ArrangementKind`.
    pub fn on_site() -> Self {
        Self(ArrangementKind::OnSite)
    }

    /// Creates an `Arrangement` with a remote `ArrangementKind`.
    pub fn remote() -> Self {
        Self(ArrangementKind::Remote)
    }

    /// Returns a copy of the contained `ArrangementKind`.
    pub fn kind(&self) -> ArrangementKind {
        self.0
    }
}