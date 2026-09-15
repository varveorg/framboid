use bitcode::serialize;
use blake3::{Hash, hash};

use crate::{account::{dossier::event::{Event, working::WorkingEvent}, errors::{VarveError, dossier::{DossierError, event::EventError}}}, addressing::Action};

impl WorkingEvent {
    /// Adds to the `WorkingEvent` with supplied actions.
    pub fn add(&mut self, actions: Vec<Action>) {
        for action in actions {
            self.0.insert(hash(&serialize(&action).unwrap()), action);
        }
    }

    /// Removes from the `WorkingEvent` with the `Hash` for the supplied `Action`.
    pub fn remove(&mut self, hash: Hash) -> Result<Action, VarveError> {
        if let Some(action) = self.0.shift_remove(&hash) {
            Ok(action)
        }
        else {
            Err(VarveError::DossierError(DossierError::EventError(EventError::IncorrectHash)))
        }
    }

    /// Completes this `WorkingEvent` by setting the start and end timestamps implicitly.
    pub fn complete(self) -> Result<Event, VarveError> {
        if self.0.is_empty() {
            return Err(VarveError::DossierError(DossierError::EventError(EventError::EmptyEvent)));
        }
        
        let from = self.0.first().unwrap().1.time;
        let to = self.0.last().unwrap().1.time;
        
        Ok(
            Event {
                actions: self.0,
                from,
                to
            }
        )
    }
}