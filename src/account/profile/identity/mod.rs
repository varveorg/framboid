pub mod name;

use crate::account::profile::identity::name::Name;

pub struct Identity {
    name: Name
}

impl Identity {
    pub fn new(name: Name) -> Self {
        Self { name }
    }
}