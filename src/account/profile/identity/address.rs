use crate::account::profile::location::Location;

pub struct Address {
    lines: Vec<String>,
    location: Location,
    postcode: String
}

impl Address {
    pub fn new(
        lines: Vec<String>,
        location: Location,
        postcode: String
    ) -> Self {
        Self {
            lines,
            location,
            postcode
        }
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn postcode(&self) -> &str {
        &self.postcode
    }
}