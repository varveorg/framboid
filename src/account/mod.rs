pub mod dossier;
pub mod profile;

use blake3::Hash;
use indexmap::IndexMap;
use time::Timestamp;

use crate::account::{dossier::Dossier, profile::Profile};

pub struct Varve {
    dossiers: IndexMap<Hash, Dossier>,
    profile: Profile,
    from: Timestamp
    // Think about adding a `to` field, will a varve ever be "completed"?
}

impl Varve {
    pub fn new() -> Varve {
        Varve {
            dossiers: IndexMap::new(),
            profile: Profile::new(),
            from: Timestamp::now()
        }
    }
}
