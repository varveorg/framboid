use crate::account::profile::location::Location;

/// A person's registration under an external scheme.
pub struct Registration {
    // Possibly make this an enum based on feedback
    scheme: String,
    identifier: String,
    jurisdiction: Location
}

impl Registration {
    /// Creates a `Registration`.
    pub fn new(
        scheme: String,
        identifier: String,
        jurisdiction: Location
    ) -> Self {
        Self {
            scheme,
            identifier,
            jurisdiction
        }
    }

    /// Returns a reference to the contained scheme.
    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    /// Returns a reference to the contained identifier.
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns a reference to the contained `Location`.
    pub fn jurisdiction(&self) -> &Location {
        &self.jurisdiction
    }
}