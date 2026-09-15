use crate::account::errors::dossier::event::EventError;

pub mod event;
pub enum DossierError {
    EmptyDossier,
    EventError(EventError),
    IncorrectHash
}