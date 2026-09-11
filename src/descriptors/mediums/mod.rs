pub mod general;
pub mod software;

use crate::descriptors::mediums::{general::GeneralMedium, software::SoftwareMedium};

// Start to enumerate this soon
pub enum Medium {
    General(GeneralMedium),
    Software(SoftwareMedium)
}