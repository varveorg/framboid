pub mod dossier;
pub mod errors;
pub mod profile;

use bitcode::serialize;
use blake3::{Hash, hash};
use indexmap::IndexMap;
use time::Timestamp;

use crate::account::{dossier::Dossier, errors::VarveError, profile::Profile};

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

    /// Adds to the `Varve` with supplied dossiers.
    pub fn add(&mut self, dossiers: Vec<Dossier>) {
        for dossier in dossiers {
            self.dossiers.insert_sorted_by_key(hash(&serialize(&dossier).unwrap()), dossier, |_, dossier| dossier.from);
        }
    }

    /// Removes from the `Varve` with the `Hash` for the supplied `Dossier`.
    pub fn remove(&mut self, hash: Hash) -> Result<Dossier, VarveError> {
        if let Some(dossier) = self.dossiers.shift_remove(&hash) {
            Ok(dossier)
        }
        else {
            Err(VarveError::IncorrectHash)
        }
    }
}
