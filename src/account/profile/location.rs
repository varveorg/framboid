pub struct Location {
    city: String,
    country: String,
    county: String,
    state: String
}

impl Location {
    pub fn new(
        city: String,
        country: String,
        county: String,
        state: String
    ) -> Self {
        Self {
            city,
            country,
            county,
            state
        }
    }
}