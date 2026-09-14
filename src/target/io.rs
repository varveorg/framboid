use std::{fs::{File, OpenOptions, read_dir, rename}, io::{Error, ErrorKind, Read, Result, Write}};

use blake3::hash;
use interprocess::local_socket::{ConnectOptions, Name, Stream};

use crate::target::{Target, TargetKind};

impl Target {
    /// A reading abstraction over `TargetKind` that fills a buffer.
    pub fn read(&self, buffer: &mut Vec<u8>) -> Result<()> {
        match &self.0 {
            TargetKind::Directory(dir) => {
                for entry in read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    
                    if !entry.file_type()?.is_file() {
                        return Err(Error::new(ErrorKind::InvalidData, "Nested directories are not allowed."))
                    }

                    if let Some(ext) = path.extension() && ext == "tmp" {
                        continue;
                    }

                    let mut file = File::open(path)?;
                    file.read_to_end(buffer)?;
                }
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

    /// A writing abstraction over `TargetKind` that consumes a buffer.
    pub fn write(&self, buffer: Vec<u8>) -> Result<()> {
        match &self.0 {
            TargetKind::Directory(dir) => {
                let name = hash(&buffer).to_string();
                
                let tmp = dir.join(format!("{}.tmp", name));
                let mut file = File::create_new(&tmp)?;
                
                file.write_all(&buffer)?;
                
                rename(tmp, dir.join(name))?;
            }
            TargetKind::Endpoint(ep) => {
                
            }
            TargetKind::File(file) => {
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(file)?;
                file.lock()?;

                file.write_all(&buffer)?;
            }
            TargetKind::Socket(name) => {
                let mut stream = connect(name)?;

                stream.write_all(&buffer)?;
            }
        }

        Ok(())
    }
}

fn connect(name: &Name<'static>) -> Result<Stream> {
    ConnectOptions::new()
        .name(name.clone())
        .connect_sync()
}