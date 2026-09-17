/// A name's primary and optional secondary identifiers.
pub struct Identifiers {
    primary: String,
    secondary: Option<String>
}

impl Identifiers {
    /// Creates an `Identifiers`.
    pub fn new(primary: String, secondary: Option<String>) -> Self {
        Self {
            primary,
            secondary
        }
    }

    /// Returns a reference to the contained primary identifier.
    pub fn primary(&self) -> &str {
        &self.primary
    }

    /// Returns a copy of the contained `Option` with a possible reference to the secondary identifier.
    pub fn secondary(&self) -> Option<&str> {
        self.secondary.as_deref()
    }
}