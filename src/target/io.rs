use std::{fs::{File, OpenOptions}, io::Result, path::PathBuf};

use crate::{addressing::Action, target::{Target, TargetKind}};

impl Target {
    pub fn read(&self) -> Result<()> {
        match &self.0 {
            TargetKind::Directory(dir) => {
                lockfile(dir)?.lock_shared()?;
            }
            TargetKind::Endpoint(ep) => {
                
            }
            TargetKind::File(file) => {
                let file = File::open(file)?;
                file.lock_shared()?;
            }
            TargetKind::Socket(name) => {
                
            }
        }

        Ok(())
    }

    pub fn write(&self, action: Action) -> Result<()> {
        match &self.0 {
            TargetKind::Directory(dir) => {
                lockfile(dir)?.lock_shared()?;
            }
            TargetKind::Endpoint(ep) => {
                
            }
            TargetKind::File(file) => {
                
            }
            TargetKind::Socket(name) => {
                
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