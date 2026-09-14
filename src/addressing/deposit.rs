use std::io::{Error, ErrorKind, Result};

use bitcode::serialize;

use crate::{addressing::Action, target::Target};

impl Action {
    /// Writes the actions to the desired `Target`.
    pub fn deposit(actions: &[Action], target: Target) -> Result<()> {
        let mut buffer = Vec::new();
        for action in actions {
            let action = serialize(action).map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
            
            buffer.extend(&action);
            buffer.extend(&(action.len() as u64).to_be_bytes());
        }
        
        target.write(buffer)
    }
}