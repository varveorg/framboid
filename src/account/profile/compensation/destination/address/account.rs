/// A type of bank account.
#[derive(Clone, Copy)]
pub struct Account(AccountKind);

/// The kind of an `Account`.
#[derive(Clone, Copy)]
pub enum AccountKind {
    Checking,
    Savings
}

impl Account {
    /// Creates an `Account` with a checking `AccountKind`.
    pub fn checking() -> Self {
        Account(AccountKind::Checking)
    }

    /// Creates an `Account` with a savings `AccountKind`.
    pub fn savings() -> Self {
        Account(AccountKind::Savings)
    }

    /// Returns a copy of the contained `AccountKind`.
    pub fn kind(&self) -> AccountKind {
        self.0
    }
}