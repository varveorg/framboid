pub mod authorization;
pub mod compensation;
pub mod experience;
pub mod identity;
pub mod location;
pub mod period;

use crate::account::profile::{authorization::Authorization, compensation::Compensation, experience::{credentials::Credential, engagement::Engagement}, identity::Identity};

pub struct Profile {
    authorization: Authorization,
    compensation: Compensation,
    credentials: Vec<Credential>,
    engagements: Vec<Engagement>,
    identity: Identity
}

impl Profile {
    pub fn new (
        authorization: Authorization,
        compensation: Compensation,
        credentials: Vec<Credential>,
        engagements: Vec<Engagement>,
        identity: Identity
    ) -> Self {
        Self {
            authorization,
            compensation,
            credentials,
            engagements,
            identity
        }
    }

    pub fn authorization(&self) -> &Authorization {
        &self.authorization
    }

    pub fn compensation(&self) -> &Compensation {
        &self.compensation
    }

    pub fn credentials(&self) -> &[Credential] {
        &self.credentials
    }

    pub fn engagements(&self) -> &[Engagement] {
        &self.engagements
    }

    pub fn identity(&self) -> &Identity {
        &self.identity
    }
}