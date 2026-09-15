use time::Date;

use crate::account::profile::experience::location::Location;

pub struct Engagement {
    employer: String,
    title: String,
    start: Date,
    end: Option<Date>,
    location: Location
}

impl Engagement {
    pub fn new(
        employer: String,
        title: String,
        start: Date,
        end: Option<Date>,
        location: Location
    ) -> Self {
        Self {
            employer,
            title,
            start,
            end,
            location
        }
    }
}