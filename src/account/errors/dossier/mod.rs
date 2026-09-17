use crate::account::errors::dossier::event::EventError;

pub mod event;
/// Errors for `Dossier`.
pub enum DossierError {
    EmptyDossier,
    EventError(EventError),
    IncorrectHash
}