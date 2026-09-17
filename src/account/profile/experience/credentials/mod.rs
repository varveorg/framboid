pub mod status;

use crate::account::profile::{experience::credentials::status::Status, location::Location};

/// A person's credential.
pub struct Credential(CredentialKind);
/// The kind of a `Credential`.
pub enum CredentialKind {
    Education {
        degree: String,
        discipline: String,
        location: Location,
        school: String,
        status: Status
    }
}

impl Credential {
    /// Creates a `Credential` with an education `CredentialKind`.
    pub fn education(
        degree: String,
        discipline: String,
        location: Location,
        school: String,
        status: Status
    ) -> Self {
        Self(
            CredentialKind::Education {
                degree,
                discipline,
                location,
                school,
                status
            }
        )
    }

    /// Returns a reference to the contained `CredentialKind`.
    pub fn kind(&self) -> &CredentialKind {
        &self.0
    }
}