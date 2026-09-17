pub struct Location {
    city: String,
    country: String,
    county: String,
    state: String
}

impl Location {
    /// Creates a `Location`.
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

    /// Returns a reference to the contained city.
    pub fn city(&self) -> &str {
        &self.city
    }

    /// Returns a reference to the contained country.
    pub fn country(&self) -> &str {
        &self.country
    }

    /// Returns a reference to the contained county.
    pub fn county(&self) -> &str {
        &self.county
    }

    /// Returns a reference to the contained state.
    pub fn state(&self) -> &str {
        &self.state
    }
}