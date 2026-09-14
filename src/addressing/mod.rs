pub mod value;

use time::Timestamp;

use crate::addressing::value::Value;

/// A single, discrete action.
pub struct Action {
    name: String,
    source: String,
    time: Timestamp,
    body: Value
}

impl Action {
    pub fn deposit(
        name: String,
        source: String,
        time: Timestamp,
        body: Value
    ) {
        
    }
}