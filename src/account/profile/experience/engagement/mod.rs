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
}