pub mod claim;
pub mod dossier;

use blake3::Hash;
use indexmap::IndexMap;
use time::Timestamp;

use crate::account::{claim::Claim, dossier::Dossier};

pub struct Varve {
    claims: IndexMap<Hash, Claim>,
    dossiers: IndexMap<Hash, Dossier>,
    from: Timestamp
    // Think about adding a `to` field, will a varve ever be "completed"?
}

impl Varve {
    pub fn new() -> Varve {
        Varve {
            claims: IndexMap::new(),
            dossiers: IndexMap::new(),
            from: Timestamp::now()
        }
    }
}
