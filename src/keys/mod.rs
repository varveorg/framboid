pub mod account;
pub mod source;

/// A trait for identifiers.
pub trait Key {
    fn new(key: String) -> Self;

    fn key(&self) -> &str;
}