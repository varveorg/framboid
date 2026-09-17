use bitcode::serialize;
use blake3::{Hash, hash};

use crate::account::{Varve, dossier::Dossier, errors::VarveError};

impl Varve {
    /// Adds to the `Varve` with supplied dossiers.
    pub fn add(&mut self, dossiers: Vec<Dossier>) {
        for dossier in dossiers {
            self.dossiers.insert_sorted_by_key(hash(&serialize(&dossier).unwrap()), dossier, |_, dossier| dossier.from);
        }
    }

    /// Removes the `Dossier` that is associated with the supplied `Hash`.
    pub fn remove(&mut self, hash: Hash) -> Result<Dossier, VarveError> {
        if let Some(dossier) = self.dossiers.shift_remove(&hash) {
            Ok(dossier)
        }
        else {
            Err(VarveError::IncorrectHash)
        }
    }
}