pub mod destination;
pub mod registration;

use crate::account::profile::compensation::{destination::Destination, registration::Registration};

/// A person's compensation destinations and registrations.
pub struct Compensation {
    destinations: Vec<Destination>,
    registrations: Vec<Registration>
}

impl Compensation {
    /// Creates a `Compensation`.
    pub fn new(
        destinations: Vec<Destination>,
        registrations: Vec<Registration>
    ) -> Self {
        Self {
            destinations,
            registrations
        }
    }

    /// Returns a reference to the contained destinations.
    pub fn destinations(&self) -> &[Destination] {
        &self.destinations
    }

    /// Returns a reference to the contained registrations.
    pub fn registrations(&self) -> &[Registration] {
        &self.registrations
    }
}