pub mod value;

use std::io::Result;

use serde::{Deserialize, Serialize};
use time::Timestamp;

use crate::{addressing::value::Value, target::Target};

/// A single, discrete action.
#[derive(Serialize, Deserialize)]
pub struct Action {
    name: String,
    source: String,
    time: Timestamp,
    body: Value
}

impl Action {
    /// Creates an `Action`.
    pub fn new(name: String, source: String, time: Timestamp, body: Value) -> Self {
        Self {
            name,
            source,
            time,
            body
        }
    }
    
    /// Writes the actions to the desired `Target`.
    pub fn deposit(actions: &[Action], target: Target) -> Result<()> {
        target.write(actions)
    }
}