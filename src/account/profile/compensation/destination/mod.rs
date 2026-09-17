pub mod address;

use crate::account::profile::compensation::destination::address::Address;

/// A destination for a person's compensation.
pub struct Destination(DestinationKind);

/// The kind of a `Destination`.
pub enum DestinationKind {
    Default(Address),
    Retirement(Address)
}

impl Destination {
    /// Creates a `Destination` with a default `DestinationKind`.
    pub fn default(address: Address) -> Self {
        Self(DestinationKind::Default(address))
    }

    /// Creates a `Destination` with a retirement `DestinationKind`.
    pub fn retirement(address: Address) -> Self {
        Self(DestinationKind::Retirement(address))
    }

    /// Returns a reference to the contained `DestinationKind`.
    pub fn kind(&self) -> &DestinationKind {
        &self.0
    }
}