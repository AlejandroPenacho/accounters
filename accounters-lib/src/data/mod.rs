pub mod account;
pub mod datetime;
pub mod transaction;

use std::collections::hash_map::Entry;
use std::{collections::HashMap, io::Write};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize, Default)]
pub struct Database {
    accounts: HashMap<account::AccountName, account::Account>,
    transactions: HashMap<transaction::TransactionId, transaction::Transaction>,
    #[serde(skip)]
    account_to_transaction_map: HashMap<account::AccountName, Vec<transaction::TransactionId>>,
}

#[derive(Debug)]
pub enum Error {
    AccountNameInUse(account::AccountName),
    TransactionIdInUse(transaction::TransactionId),
    UnknownAssociatedAccount((transaction::TransactionId, account::AccountName)),
}

impl Database {
    pub fn add_account(&mut self, new_acc: account::Account) -> Result<(), Error> {
        // Check that the account name does not already exist. If it does not,
        // add the account name to the account map.
        match self.accounts.entry(new_acc.get_name().to_owned()) {
            Entry::Occupied(_) => {
                return Err(Error::AccountNameInUse(new_acc.get_name().to_owned()))
            }
            Entry::Vacant(entry) => {
                self.account_to_transaction_map
                    .insert(new_acc.get_name().to_owned(), vec![]);
                entry.insert(new_acc);
            }
        }

        Ok(())
    }

    pub fn add_transaction(&mut self, new_trns: transaction::Transaction) -> Result<(), Error> {
        if self.transactions.get(&new_trns.get_id()).is_some() {
            return Err(Error::TransactionIdInUse(new_trns.get_id()));
        }

        for account_name in new_trns.get_associated_accounts() {
            if self.accounts.get(account_name).is_none() {
                return Err(Error::UnknownAssociatedAccount((
                    new_trns.get_id(),
                    account_name.to_owned(),
                )));
            }
        }

        for account_name in new_trns.get_associated_accounts() {
            self.account_to_transaction_map
                .get_mut(account_name)
                .unwrap()
                .push(new_trns.get_id())
        }

        self.transactions.insert(new_trns.get_id(), new_trns);

        Ok(())
    }

    /*
    pub fn get_account_balance(
        &self,
        account_name: &account::AccountName,
        start: Option<datetime::DateTime>,
        end: Option<datetime::DateTime>,
    ) -> HashMap<transaction::Currency, transaction::Amount> {
        let mut currencies = HashMap::new();

        for trns_id in self.account_to_transaction_map.get(account_name).iter() {
            let amount = self.transactions.get(trns_id);
            unimplemented!();
        }
    }
    */
}

impl Database {
    pub fn save_to_file(&self, filename: &str) {
        let text = serde_json::to_string_pretty(self).unwrap();

        let mut file = std::fs::File::create(filename).unwrap();

        file.write_all(text.as_bytes()).unwrap();
    }

    pub fn read_from_file(filename: &str) -> Result<Self, &'static str> {
        let text = std::fs::read_to_string(filename).map_err(|_| "The file does not exist")?;

        let mut database: Database =
            serde_json::from_str(&text).map_err(|_| "Erros in deserialization!!")?;

        database
            .build_account_transaction_map()
            .map_err(|_| "Whatever")?;

        Ok(database)
    }

    fn build_account_transaction_map(&mut self) -> Result<(), Error> {
        for acc_name in self.accounts.keys() {
            self.account_to_transaction_map
                .insert(acc_name.to_owned(), vec![]);
        }
        for (trns_id, transaction) in self.transactions.iter() {
            for acc_name in transaction.get_associated_accounts() {
                self.account_to_transaction_map
                    .get_mut(acc_name)
                    .unwrap()
                    .push(*trns_id);
            }
        }

        Ok(())
    }
}
