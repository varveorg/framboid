pub mod dossier;

use blake3::Hash;
use indexmap::IndexMap;
use time::Timestamp;

use crate::account::dossier::Dossier;

pub struct Varve {
    dossiers: IndexMap<Hash, Dossier>,
    from: Timestamp
    // Think about adding a `to` field, will a varve ever be "completed"?
}

impl Varve {
    pub fn new() -> Varve {
        Varve {
            dossiers: IndexMap::new(),
            from: Timestamp::now()
        }
    }
}
