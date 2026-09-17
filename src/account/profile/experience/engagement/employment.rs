pub struct Employment(EmploymentKind);
enum EmploymentKind {
    Apprenticeship,
    Contract,
    Internship,
    FullTime,
    PartTime
}

impl Employment {
    pub fn apprenticeship() -> Self {
        Self(EmploymentKind::Apprenticeship)
    }

    pub fn contract() -> Self {
        Self(EmploymentKind::Contract)
    }

    pub fn internship() -> Self {
        Self(EmploymentKind::Internship)
    }

    pub fn full_time() -> Self {
        Self(EmploymentKind::FullTime)
    }

    pub fn part_time() -> Self {
        Self(EmploymentKind::PartTime)
    }
}