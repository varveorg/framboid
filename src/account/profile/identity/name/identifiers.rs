pub struct Identifiers {
    primary: String,
    secondary: Option<String>
}

impl Identifiers {
    pub fn new(primary: String, secondary: Option<String>) -> Self {
        Self {
            primary,
            secondary
        }
    }

    pub fn primary(&self) -> &str {
        &self.primary
    }

    pub fn secondary(&self) -> Option<&str> {
        self.secondary.as_deref()
    }
}