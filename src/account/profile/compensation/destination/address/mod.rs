pub mod account;

use crate::account::profile::compensation::destination::address::account::Account;

/// An address a payment is sent to.
pub struct Address(AddressKind);

/// The kind of an `Address`.
pub enum AddressKind {
    Ach {
        account: String,
        kind: Account,
        routing: String,
    },
    Iban(String),
    Pix(String),
    Upi(String)
}

impl Address {
    /// Creates an `Address` with an ACH `AddressKind`.
    pub fn ach(
        account: String,
        routing: String,
        kind: Account
    ) -> Self {
        Self(
            AddressKind::Ach {
                account,
                routing,
                kind
            }
        )
    }

    /// Creates an `Address` with an IBAN `AddressKind`.
    pub fn iban(iban: String) -> Self {
        Self(AddressKind::Iban(iban))
    }

    /// Creates an `Address` with a Pix `AddressKind`.
    pub fn pix(key: String) -> Self {
        Self(AddressKind::Pix(key))
    }

    /// Creates an `Address` with a UPI `AddressKind`.
    pub fn upi(vpa: String) -> Self {
        Self(AddressKind::Upi(vpa))
    }

    /// Returns a reference to the contained `AddressKind`.
    pub fn kind(&self) -> &AddressKind {
        &self.0
    }
}