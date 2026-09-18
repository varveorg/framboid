/// An account identifier.
pub struct AccountKey(String);

impl AccountKey {
    /// Creates an `AccountKey`.
    pub fn new(key: String) -> Self {
        AccountKey(key)
    }

    /// Returns a reference to the contained key.
    pub fn key(&self) -> &str {
        &self.0
    }
}