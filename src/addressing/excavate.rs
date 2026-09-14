use std::io::{Error, ErrorKind, Result};

use bitcode::deserialize;

use crate::{addressing::Action, target::Target};

impl Action {
    pub fn excavate(target: Target) -> Result<Vec<Action>> {
        let mut buffer: Vec<u8> = Vec::new();
        target.read(&mut buffer)?;
    
        let mut actions: Vec<Action> = Vec::new();
        let mut start = buffer.len();
        while !buffer.is_empty() {
            start -= 8;
            let len = u64::from_be_bytes(buffer[start..].try_into().unwrap());
            buffer.truncate(start);
    
            start -= len as usize;
            actions.push(deserialize(buffer[start..].try_into().unwrap()).map_err(|err| Error::new(ErrorKind::InvalidData, err))?);
            buffer.truncate(start);
        }
        
        Ok(actions)
    }
}