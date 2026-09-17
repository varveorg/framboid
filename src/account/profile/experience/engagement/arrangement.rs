#[derive(Clone, Copy)]
pub struct Arrangement(ArrangementKind);

#[derive(Clone, Copy)]
pub enum ArrangementKind {
    Hybrid,
    OnSite,
    Remote
}

impl Arrangement {
    pub fn hybrid() -> Self {
        Self(ArrangementKind::Hybrid)
    }

    pub fn on_site() -> Self {
        Self(ArrangementKind::OnSite)
    }

    pub fn remote() -> Self {
        Self(ArrangementKind::Remote)
    }

    pub fn kind(&self) -> ArrangementKind {
        self.0
    }
}