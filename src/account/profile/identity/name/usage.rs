use crate::account::profile::period::Period;

pub struct Usage(UsageKind);
enum UsageKind {
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
}