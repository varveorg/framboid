use crate::account::profile::location::Location;

pub struct Issuer {
    name: String,
    jurisdiction: Location
}

impl Issuer {
    pub fn new(
        name: String,
        jurisdiction: Location
    ) -> Self {
        Self {
            name,
            jurisdiction
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn jurisdiction(&self) -> &Location {
        &self.jurisdiction
    }
}