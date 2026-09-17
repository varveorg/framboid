#[derive(Clone, Copy)]
pub struct Account(AccountKind);

#[derive(Clone, Copy)]
pub enum AccountKind {
    Checking,
    Savings
}

impl Account {
    pub fn checking() -> Self {
        Account(AccountKind::Checking)
    }

    pub fn savings() -> Self {
        Account(AccountKind::Savings)
    }

    pub fn kind(&self) -> AccountKind {
        self.0
    }
}