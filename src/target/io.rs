use std::{fs::{File, OpenOptions}, io::{Error, ErrorKind, Read, Result, Write}, path::PathBuf};

use bitcode::serialize;
use interprocess::local_socket::{ConnectOptions, Name, Stream};

use crate::{addressing::Action, target::{Target, TargetKind}};

impl Target {
    pub fn read(&self, buffer: &mut Vec<u8>) -> Result<()> {
        match &self.0 {
            TargetKind::Directory(dir) => {
                lockfile(dir)?.lock_shared()?;
            }
            TargetKind::Endpoint(ep) => {
                
            }
            TargetKind::File(file) => {
                let mut file = File::open(file)?;
                file.lock_shared()?;

                file.read_to_end(buffer)?;
            }
            TargetKind::Socket(name) => {
                let mut stream = connect(name)?;

                stream.read_to_end(buffer)?;
            }
        }

        Ok(())
    }

    pub fn write(&self, actions: &[Action]) -> Result<()> {
        match &self.0 {
            TargetKind::Directory(dir) => {
                lockfile(dir)?.lock_shared()?;
            }
            TargetKind::Endpoint(ep) => {
                
            }
            TargetKind::File(file) => {
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(file)?;
                file.lock()?;

                write(&mut file, actions)?;
            }
            TargetKind::Socket(name) => {
                let mut stream = connect(name)?;

                write(&mut stream, actions)?;
            }
        }

        Ok(())
    }
}

fn lockfile(path: &PathBuf) -> Result<File> {
    OpenOptions::new()
        .create(true)
        .open(path.join(".lock"))
}

fn connect(name: &Name<'static>) -> Result<Stream> {
    ConnectOptions::new()
        .name(name.clone())
        .connect_sync()
}

fn write<W: Write>(writer: &mut W, actions: &[Action]) -> Result<()> {
    for action in actions {
        let action = serialize(action).map_err(|err| Error::new(ErrorKind::InvalidData, err))?;

        writer.write_all(&action)?;
        writer.write_all(&(action.len() as u64).to_be_bytes())?;
    }

    Ok(())
}