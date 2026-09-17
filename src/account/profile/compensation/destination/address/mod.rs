pub mod account;

use crate::account::profile::compensation::destination::address::account::Account;

pub struct Address(AddressKind);

pub enum AddressKind {
    Ach {
        account: String,
        routing: String,
        kind: Account
    },
    Iban(String),
    Upi(String),
    Pix(String)
}

impl Address {
    pub fn ach(
        account: String,
        routing: String,
        kind: Account
    ) -> Self {
        Self(AddressKind::Ach {
            account,
            routing,
            kind
        })
    }

    pub fn iban(iban: String) -> Self {
        Self(AddressKind::Iban(iban))
    }

    pub fn upi(vpa: String) -> Self {
        Self(AddressKind::Upi(vpa))
    }

    pub fn pix(key: String) -> Self {
        Self(AddressKind::Pix(key))
    }

    pub fn kind(&self) -> &AddressKind {
        &self.0
    }
}