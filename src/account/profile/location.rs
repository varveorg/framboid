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

    pub fn city(&self) -> &str {
        &self.city
    }

    pub fn country(&self) -> &str {
        &self.country
    }

    pub fn county(&self) -> &str {
        &self.county
    }

    pub fn state(&self) -> &str {
        &self.state
    }
}