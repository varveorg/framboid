use crate::account::profile::location::Location;

/// A party that issued a `Document`.
pub struct Issuer {
    name: String,
    jurisdiction: Location
}

impl Issuer {
    /// Creates an `Issuer`.
    pub fn new(
        name: String,
        jurisdiction: Location
    ) -> Self {
        Self {
            name,
            jurisdiction
        }
    }

    /// Returns a reference to the contained name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a reference to the contained `Location`.
    pub fn jurisdiction(&self) -> &Location {
        &self.jurisdiction
    }
}