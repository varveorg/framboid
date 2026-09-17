use crate::account::profile::location::Location;

pub struct Registration {
    // Possibly make this an enum based on feedback
    scheme: String,
    identifier: String,
    jurisdiction: Location
}

impl Registration {
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

    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn jurisdiction(&self) -> &Location {
        &self.jurisdiction
    }
}