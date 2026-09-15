pub struct Location {
    city: String,
    country: String
}

impl Location {
    pub fn new(city: String, country: String) -> Self {
        Self {
            city,
            country
        }
    }
}