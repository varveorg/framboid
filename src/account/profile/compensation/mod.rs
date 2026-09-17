pub mod destination;
pub mod registration;

use crate::account::profile::compensation::{destination::Destination, registration::Registration};

pub struct Compensation {
    destinations: Vec<Destination>,
    registrations: Vec<Registration>
}

impl Compensation {
    pub fn new(
        destinations: Vec<Destination>,
        registrations: Vec<Registration>
    ) -> Self {
        Self {
            destinations,
            registrations
        }
    }

    pub fn destinations(&self) -> &[Destination] {
        &self.destinations
    }

    pub fn registrations(&self) -> &[Registration] {
        &self.registrations
    }
}