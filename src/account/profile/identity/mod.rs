pub mod address;
pub mod name;

use time::Date;

use crate::account::profile::identity::{address::Address, name::Name};

pub struct Identity {
    address: Address,
    birth: Date,
    emails: Vec<String>,
    names: Vec<Name>,
    phones: Vec<String>,
}

impl Identity {
    /// Creates an `Identity`.
    pub fn new(
        address: Address,
        birth: Date,
        emails: Vec<String>,
        names: Vec<Name>,
        phones: Vec<String>
    ) -> Self {
        Self {
            address,
            birth,
            emails,
            names,
            phones
        }
    }

    /// Returns a reference to the contained `Address`.
    pub fn address(&self) -> &Address {
        &self.address
    }

    /// Returns a copy of the contained birth `Date`.
    pub fn birth(&self) -> Date {
        self.birth
    }

    /// Returns a reference to the contained emails.
    pub fn emails(&self) -> &[String] {
        &self.emails
    }

    /// Returns a reference to the contained names.
    pub fn names(&self) -> &[Name] {
        &self.names
    }

    /// Returns a reference to the contained phone numbers.
    pub fn phones(&self) -> &[String] {
        &self.phones
    }
}