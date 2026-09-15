pub struct Arrangement (ArrangementKind);
enum ArrangementKind {
    Hybrid,
    OnSite,
    Remote
}

impl Arrangement {
    pub fn hybrid () -> Self {
        Self (ArrangementKind::Hybrid)
    }

    pub fn on_site () -> Self {
        Self (ArrangementKind::OnSite)
    }

    pub fn remote () -> Self {
        Self (ArrangementKind::Remote)
    }
}