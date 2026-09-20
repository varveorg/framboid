pub mod authorization;
pub mod compensation;
pub mod credentials;
pub mod engagement;
pub mod identity;
pub mod location;
pub mod period;

use crate::account::profile::{authorization::Authorization, compensation::Compensation, credentials::Credential, engagement::Engagement, identity::Identity};

/// A set of information a person supplies about themselves.
pub struct Profile {
    authorization: Authorization,
    compensation: Compensation,
    credentials: Vec<Credential>,
    engagements: Vec<Engagement>,
    identity: Identity
}

impl Profile {
    /// Creates a `Profile`.
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

    /// Returns a reference to the contained `Authorization`.
    pub fn authorization(&self) -> &Authorization {
        &self.authorization
    }

    /// Returns a reference to the contained `Compensation`.
    pub fn compensation(&self) -> &Compensation {
        &self.compensation
    }

    /// Returns a reference to the contained credentials.
    pub fn credentials(&self) -> &[Credential] {
        &self.credentials
    }

    /// Returns a reference to the contained engagements.
    pub fn engagements(&self) -> &[Engagement] {
        &self.engagements
    }

    /// Returns a reference to the contained `Identity`.
    pub fn identity(&self) -> &Identity {
        &self.identity
    }
}