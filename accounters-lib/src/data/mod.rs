pub mod account;
pub mod datetime;
pub mod transaction;

use std::collections::hash_map::Entry;
use std::{collections::HashMap, io::Write};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize, Default)]
pub struct Database {
    accounts: Vec<account::Account>,
    transactions: Vec<transaction::Transaction>,
    #[serde(skip)]
    accounts_map: HashMap<account::AccountName, usize>,
    #[serde(skip)]
    transactions_map: HashMap<transaction::TransactionId, usize>,
    #[serde(skip)]
    account_to_transaction_map: HashMap<account::AccountName, Vec<transaction::TransactionId>>,
}

impl Database {
    pub fn add_account(&mut self, new_acc: account::Account) -> Result<(), &'static str> {
        // Check that the account name does not already exist. If it does not,
        // add the account name to the account map.
        match self.accounts_map.entry(new_acc.get_name().to_owned()) {
            Entry::Occupied(_) => return Err("Account name already exists"),
            Entry::Vacant(entry) => {
                entry.insert(self.accounts.len());
            }
        }
        self.account_to_transaction_map
            .insert(new_acc.get_name().to_owned(), vec![]);

        // Add the account to the vector.
        self.accounts.push(new_acc);
        Ok(())
    }

    pub fn add_transaction(
        &mut self,
        new_trns: transaction::Transaction,
    ) -> Result<(), &'static str> {
        if self.transactions_map.get(&new_trns.get_id()).is_some() {
            return Err("Transacion id already in use?!?!");
        }

        for account_name in new_trns.get_associated_accounts() {
            if self.accounts_map.get(account_name).is_none() {
                return Err("One of the accounts in the transaction does not exist");
            }
        }

        self.transactions_map
            .insert(new_trns.get_id(), self.transactions.len());

        for account_name in new_trns.get_associated_accounts() {
            self.account_to_transaction_map
                .get_mut(account_name)
                .unwrap()
                .push(new_trns.get_id())
        }

        self.transactions.push(new_trns);
        Ok(())
    }
}

impl Database {
    pub fn save_to_file(&self, filename: &str) {
        let text = serde_json::to_string(self).unwrap();

        let mut file = std::fs::File::create(filename).unwrap();

        file.write_all(text.as_bytes()).unwrap();
    }

    pub fn read_from_file(filename: &str) -> Result<Self, &'static str> {
        let text = std::fs::read_to_string(filename).map_err(|_| "The file does not exist")?;

        let mut database: Database =
            serde_json::from_str(&text).map_err(|_| "Erros in deserialization!!")?;

        database.build_maps()?;

        Ok(database)
    }

    fn build_maps(&mut self) -> Result<(), &'static str> {
        for (index, account) in self.accounts.iter().enumerate() {
            match self.accounts_map.entry(account.get_name().clone()) {
                Entry::Occupied(_) => return Err("Same account name twice"),
                Entry::Vacant(entry) => {
                    entry.insert(index);
                }
            }

            self.account_to_transaction_map
                .insert(account.get_name().to_owned(), vec![]);
        }

        for (index, transaction) in self.transactions.iter().enumerate() {
            match self.transactions_map.entry(transaction.get_id()) {
                Entry::Occupied(_) => return Err("Same transaction id twice"),
                Entry::Vacant(entry) => {
                    entry.insert(index);
                }
            }

            for account_name in transaction.get_associated_accounts() {
                self.account_to_transaction_map
                    .get_mut(account_name)
                    .unwrap()
                    .push(transaction.get_id())
            }
        }

        Ok(())
    }
}
