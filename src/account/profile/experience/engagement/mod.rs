pub mod arrangement;
pub mod employment;

use crate::account::profile::{experience::engagement::{arrangement::Arrangement, employment::Employment}, location::Location, period::Period};

/// A person's engagement with an employer.
pub struct Engagement {
    arrangement: Arrangement,
    employer: String,
    employment: Employment,
    location: Option<Location>,
    period: Period,
    title: String
}

impl Engagement {
    /// Creates an `Engagement`.
    pub fn new(
        arrangement: Arrangement,
        employer: String,
        employment: Employment,
        location: Option<Location>,
        period: Period,
        title: String
    ) -> Self {
        Self {
            arrangement,
            employer,
            employment,
            location,
            period,
            title
        }
    }

    /// Returns a copy of the contained `Arrangement`.
    pub fn arrangement(&self) -> Arrangement {
        self.arrangement
    }

    /// Returns a reference to the contained employer.
    pub fn employer(&self) -> &str {
        &self.employer
    }

    /// Returns a copy of the contained `Employment`.
    pub fn employment(&self) -> Employment {
        self.employment
    }

    /// Returns a copy of the contained `Option` with a possible reference to the `Location`.
    pub fn location(&self) -> Option<&Location> {
        self.location.as_ref()
    }

    /// Returns a copy of the contained `Period`.
    pub fn period(&self) -> Period {
        self.period
    }

    /// Returns a reference to the contained title.
    pub fn title(&self) -> &str {
        &self.title
    }
}