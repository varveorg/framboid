pub mod value;

use std::io::Result;

use time::Timestamp;

use crate::{addressing::value::Value, target::Target};

/// A single, discrete action.
pub struct Action {
    name: String,
    source: String,
    time: Timestamp,
    body: Value
}

impl Action {
    /// Creates an `Action` and serializes it, sending it to the desired `Target`.
    pub fn deposit(
        name: String,
        source: String,
        time: Timestamp,
        body: Value,
        target: Target
    ) -> Result<()> {
        target.write(Action { name, source, time, body })
    }
}