pub mod issuer;

use std::range::Range;

use time::Date;

use crate::account::profile::authorization::document::issuer::Issuer;

pub struct Document {
    issuer: Issuer,
    name: String,
    number: Option<String>,
    validity: Option<Range<Date>>
}

impl Document {
    pub fn new(
        issuer: Issuer,
        name: String,
        number: Option<String>,
        validity: Option<Range<Date>>
    ) -> Self {
        Self {
            issuer,
            name,
            number,
            validity
        }
    }

    pub fn issuer(&self) -> &Issuer {
        &self.issuer
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn number(&self) -> Option<&str> {
        self.number.as_deref()
    }

    pub fn validity(&self) -> Option<Range<Date>> {
        self.validity
    }
}