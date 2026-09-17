pub mod dossier;
pub mod errors;
pub mod operations;
pub mod profile;

use blake3::Hash;
use indexmap::IndexMap;
use time::Timestamp;

use crate::account::{dossier::Dossier, profile::Profile};

/// A person's complete, accumulating record.
pub struct Varve {
    dossiers: IndexMap<Hash, Dossier>,
    profile: Profile,
    from: Timestamp
    // `to` field doesn't exist because a `Varve` never completes.
}

impl Varve {
    /// Creates a `Varve` originating at the current time.
    pub fn new(profile: Profile) -> Self {
        Self {
            dossiers: IndexMap::new(),
            profile,
            from: Timestamp::now()
        }
    }

    /// Returns a reference to the contained dossiers.
    pub fn dossiers(&self) -> &IndexMap<Hash, Dossier> {
        &self.dossiers
    }

    /// Returns a reference to the contained `Profile`.
    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    /// Returns a copy of the contained origin `Timestamp`.
    pub fn from(&self) -> Timestamp {
        self.from
    }
}
