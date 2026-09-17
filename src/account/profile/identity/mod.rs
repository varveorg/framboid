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

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn birth(&self) -> Date {
        self.birth
    }

    pub fn emails(&self) -> &[String] {
        &self.emails
    }

    pub fn names(&self) -> &[Name] {
        &self.names
    }

    pub fn phones(&self) -> &[String] {
        &self.phones
    }
}