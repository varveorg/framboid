pub mod authorization;
pub mod compensation;
pub mod credentials;
pub mod engagements;
pub mod identity;

use crate::account::profile::{authorization::Authorization, compensation::Compensation, credentials::Credential, engagements::Engagement, identity::Identity};

pub struct Profile {
    authorization: Authorization,
    compensation: Compensation,
    credentials: Vec<Credential>,
    engagements: Vec<Engagement>,
    identity: Identity
}

impl Profile {
    pub fn new() -> Self {
        Profile {
            authorization: Authorization::new(),
            compensation: Compensation::new(),
            credentials: Vec::new(),
            engagements: Vec::new(),
            identity: Identity::new()
        }
    }
}