use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Account {
    name: AccountName,
    account_type: AccountType,
}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum AccountType {
    Asset,
    Flow,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AccountName(String);

impl AccountName {
    pub fn new(name: &str) -> Self {
        Self(name.to_owned())
    }
}

impl Account {
    pub fn new(account_name: &str, account_type: AccountType) -> Self {
        Account {
            name: AccountName(account_name.to_owned()),
            account_type,
        }
    }

    pub fn get_name(&self) -> &AccountName {
        &self.name
    }
}
