pub mod education;

use crate::account::profile::experience::{credentials::education::{Education, Status}, location::Location};

pub struct Credential(CredentialKind);
enum CredentialKind {
    Education(Education)
}

impl Credential {
    pub fn education(
        degree: String,
        school: String,
        status: Status,
        location: Location,
        discipline: String
    ) -> Self {
        Credential(
            CredentialKind::Education(
                Education {
                    degree,
                    school,
                    status,
                    location,
                    discipline
                }
            )
        )
    }
}