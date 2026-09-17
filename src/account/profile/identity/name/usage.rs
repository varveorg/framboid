use crate::account::profile::period::Period;

#[derive(Clone, Copy)]
pub struct Usage(UsageKind);

#[derive(Clone, Copy)]
pub enum UsageKind {
    Legal,
    Prior(Period),
    Used
}

impl Usage {
    pub fn legal() -> Self {
        Self(UsageKind::Legal)
    }

    pub fn prior(period: Period) -> Self {
        Self(UsageKind::Prior(period))
    }

    pub fn used() -> Self {
        Self(UsageKind::Used)
    }

    pub fn kind(&self) -> UsageKind {
        self.0
    }
}