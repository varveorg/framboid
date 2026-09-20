/// A form of employment.
#[derive(Clone, Copy)]
pub struct Employment(EmploymentKind);

/// The kind of an `Employment`.
#[derive(Clone, Copy)]
pub enum EmploymentKind {
    Apprenticeship,
    Contract,
    Internship,
    FullTime,
    PartTime
}

impl Employment {
    /// Creates an `Employment` with an apprenticeship `EmploymentKind`.
    pub fn apprenticeship() -> Self {
        Self(EmploymentKind::Apprenticeship)
    }

    /// Creates an `Employment` with a contract `EmploymentKind`.
    pub fn contract() -> Self {
        Self(EmploymentKind::Contract)
    }

    /// Creates an `Employment` with an internship `EmploymentKind`.
    pub fn internship() -> Self {
        Self(EmploymentKind::Internship)
    }

    /// Creates an `Employment` with a full-time `EmploymentKind`.
    pub fn full_time() -> Self {
        Self(EmploymentKind::FullTime)
    }

    /// Creates an `Employment` with a part-time `EmploymentKind`.
    pub fn part_time() -> Self {
        Self(EmploymentKind::PartTime)
    }

    /// Returns a copy of the contained `EmploymentKind`.
    pub fn kind(&self) -> EmploymentKind {
        self.0
    }
}