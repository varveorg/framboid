pub mod address;

use crate::account::profile::compensation::destination::address::Address;

pub struct Destination(DestinationKind);

pub enum DestinationKind {
    Default(Address),
    Retirement(Address)
}

impl Destination {
    pub fn default(address: Address) -> Self {
        Self(DestinationKind::Default(address))
    }

    pub fn retirement(address: Address) -> Self {
        Self(DestinationKind::Retirement(address))
    }

    pub fn kind(&self) -> &DestinationKind {
        &self.0
    }
}