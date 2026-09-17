use crate::account::profile::location::Location;

/// A person's postal address.
pub struct Address {
    lines: Vec<String>,
    location: Location,
    postcode: String
}

impl Address {
    /// Creates an `Address`.
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

    /// Returns a reference to the contained address lines.
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    /// Returns a reference to the contained `Location`.
    pub fn location(&self) -> &Location {
        &self.location
    }

    /// Returns a reference to the contained postcode.
    pub fn postcode(&self) -> &str {
        &self.postcode
    }
}