pub mod identifiers;
pub mod usage;

use crate::account::profile::identity::name::{identifiers::Identifiers, usage::Usage};

pub struct Name {
    latin: Option<Identifiers>,
    native: Identifiers,
    usage: Usage
}

impl Name {
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
}