pub mod issuer;

use std::range::Range;

use time::Date;

use crate::account::profile::authorization::document::issuer::Issuer;

/// A document authorizing a person.
pub struct Document {
    issuer: Issuer,
    name: String,
    number: Option<String>,
    validity: Option<Range<Date>>
}

impl Document {
    /// Creates a `Document`.
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

    /// Returns a reference to the contained `Issuer`.
    pub fn issuer(&self) -> &Issuer {
        &self.issuer
    }

    /// Returns a reference to the contained name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a copy of the contained `Option` with a possible reference to the number.
    pub fn number(&self) -> Option<&str> {
        self.number.as_deref()
    }

    /// Returns a copy of the contained `Option` with a possible copy of the validity range.
    pub fn validity(&self) -> Option<Range<Date>> {
        self.validity
    }
}