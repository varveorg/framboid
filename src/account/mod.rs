pub mod dossier;
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
    pub fn new() -> Self {
        Self {
            dossiers: IndexMap::new(),
            profile: Profile::new(),
            from: Timestamp::now()
        }
    }

    pub fn dossiers(&self) -> &IndexMap<Hash, Dossier> {
        &self.dossiers
    }

    pub fn dossiers_mut(&mut self) -> &mut IndexMap<Hash, Dossier> {
        &mut self.dossiers
    }

    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    pub fn profile_mut(&mut self) -> &mut Profile {
        &mut self.profile
    }
}
