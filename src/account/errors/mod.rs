pub mod dossier;
pub mod profile;

use crate::account::errors::{dossier::DossierError, profile::ProfileError};

pub enum VarveError {
    DossierError(DossierError),
    IncorrectHash,
    ProfileError(ProfileError)
}