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
}