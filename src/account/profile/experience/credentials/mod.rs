pub mod status;

use crate::account::profile::{experience::credentials::status::Status, location::Location};

pub struct Credential(CredentialKind);
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

    pub fn kind(&self) -> &CredentialKind {
        &self.0
    }
}