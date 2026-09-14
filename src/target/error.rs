use std::path::PathBuf;

/// Errors for `Target`.
pub enum TargetError {
    InvalidDirectory(PathBuf),
    InvalidEndpoint(String),
    InvalidFile(PathBuf),
    InvalidSocket(String)
}