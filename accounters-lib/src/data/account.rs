use serde::{Deserialize, Serialize};

use std::collections::HashSet;

use crate::data::transaction::TransactionId;

#[derive(Deserialize, Serialize)]
pub struct Account {
    name: AccountName,
    account_type: AccountType,
    #[serde(skip)]
    transactions: HashSet<TransactionId>,
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
            transactions: HashSet::default(),
        }
    }

    pub fn add_transaction(&mut self, id: TransactionId) {
        self.transactions.insert(id);
    }

    pub fn get_name(&self) -> &AccountName {
        &self.name
    }
}
