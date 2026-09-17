pub mod arrangement;
pub mod employment;

use crate::account::profile::{experience::engagement::{arrangement::Arrangement, employment::Employment}, location::Location, period::Period};

pub struct Engagement {
    arrangement: Arrangement,
    employer: String,
    employment: Employment,
    location: Option<Location>,
    period: Period,
    title: String
}

impl Engagement {
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

    pub fn arrangement(&self) -> Arrangement {
        self.arrangement
    }

    pub fn employer(&self) -> &str {
        &self.employer
    }

    pub fn employment(&self) -> Employment {
        self.employment
    }

    pub fn location(&self) -> Option<&Location> {
        self.location.as_ref()
    }

    pub fn period(&self) -> Period {
        self.period
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}