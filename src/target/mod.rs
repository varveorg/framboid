pub mod endpoint;
pub mod error;
pub mod io;

use std::path::PathBuf;

use interprocess::local_socket::{GenericNamespaced, Name, ToNsName};

use crate::target::{endpoint::Endpoint, error::TargetError};

/// A source or destination for deposit and excavation.
pub struct Target(TargetKind);
enum TargetKind {
    Directory(PathBuf),
    Endpoint(Endpoint),
    File(PathBuf),
    Socket(Name<'static>)
}

impl Target {
    /// Validates directory and converts into PathBuf (if necessary).
    pub fn directory(path: impl Into<PathBuf>) -> Result<Self, TargetError> {
        let path = path.into();

        if !path.is_dir() {
            return Err(TargetError::InvalidDirectory(path));
        }

        Ok(Target(TargetKind::Directory(path)))
    }

    /// Validates endpoint and converts into whatever format necessary.
    pub fn endpoint(endpoint: Endpoint) -> Self {
        Target(TargetKind::Endpoint(endpoint))
    }

    /// Validates file and converts into PathBuf (if necessary).
    pub fn file(path: impl Into<PathBuf>) -> Result<Self, TargetError> {
        let path = path.into();

        if !path.is_file() {
            return Err(TargetError::InvalidFile(path));
        }

        Ok(Target(TargetKind::File(path)))
    }

    /// Validates socket/pipe name and converts into `Name<'static>`.
    pub fn socket(name: String) -> Result<Self, TargetError> {
        if let Err(_) = name.as_str().to_ns_name::<GenericNamespaced>() {
            return Err(TargetError::InvalidSocket(name));
        }

        let name = name.to_ns_name::<GenericNamespaced>().unwrap();
        
        Ok(Target(TargetKind::Socket(name)))
    }
}