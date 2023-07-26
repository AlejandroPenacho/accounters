use serde::{Deserialize, Serialize};

use std::collections::HashSet;

use crate::data::transaction::TransactionId;
use crate::data::tags::Tag;

#[derive(Deserialize, Serialize)]
pub struct Account {
    name: AccountName,
    account_type: AccountType,
    tags: HashSet<Tag>,
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
            tags: HashSet::new(),
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
    
    pub fn get_transaction_ids(&self) -> impl Iterator<Item=&TransactionId> {
        self.transactions.iter()
    }

    pub fn remove_transaction(&mut self, transaction_id: &TransactionId) -> Result<(), &'static str> {
        let was_there = self.transactions.remove(transaction_id);
        if was_there {
            Ok(())
        } else {
            Err("Transaction ID not associated")
        }
    }

    pub fn has_transactions(&self) -> bool {
        !self.transactions.is_empty()
    }
}
