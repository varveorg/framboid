pub mod identifiers;
pub mod usage;

use crate::account::profile::identity::name::{identifiers::Identifiers, usage::Usage};

/// A person's name in native and optional latin `Identifiers`.
pub struct Name {
    latin: Option<Identifiers>,
    native: Identifiers,
    usage: Usage
}

impl Name {
    /// Creates a `Name`.
    pub fn new(
        latin: Option<Identifiers>,
        native: Identifiers,
        usage: Usage
    ) -> Self {
        Self {
            latin,
            native,
            usage
        }
    }

    /// Returns a copy of the contained `Option` with a possible reference to the latin `Identifiers`.
    pub fn latin(&self) -> Option<&Identifiers> {
        self.latin.as_ref()
    }

    /// Returns a reference to the contained native `Identifiers`.
    pub fn native(&self) -> &Identifiers {
        &self.native
    }

    /// Returns a copy of the contained `Usage`.
    pub fn usage(&self) -> Usage {
        self.usage
    }
}