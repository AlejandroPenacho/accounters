pub mod account;
pub mod datetime;
pub mod money;
pub mod transaction;
pub mod tags;

use std::collections::hash_map::Entry;
use std::{collections::HashMap, io::Write};

use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use serde_json;

#[derive(Deserialize, Serialize, Default)]
pub struct Database {
    #[serde(
        serialize_with = "serialize_accounts",
        deserialize_with = "deserialize_accounts"
    )]
    accounts: HashMap<account::AccountName, account::Account>,
    #[serde(
        serialize_with = "serialize_transactions",
        deserialize_with = "deserialize_transactions"
    )]
    transactions: HashMap<transaction::TransactionId, transaction::Transaction>,
}

#[derive(Debug)]
pub enum Error {
    AccountNameInUse(account::AccountName),
    AccountHasTransactions(account::AccountName),
    TransactionIdInUse(transaction::TransactionId),
    UnknownAccount(account::AccountName),
    UnknownTransaction(transaction::TransactionId),
    AccountNotAssociatedWithTransaction((account::AccountName, transaction::TransactionId)),
    UnbalancedTransaction
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
                entry.insert(new_acc);
            }
        }

        Ok(())
    }

    pub fn remove_account(&mut self, account_name: account::AccountName) -> Result<(), Error> {
        let Some(account) = self.accounts.get(&account_name) else {
            return Err(Error::UnknownAccount(account_name))
        };

        if account.has_transactions() {
            return Err(Error::AccountHasTransactions(account_name))
        }

        self.accounts.remove(&account_name);

        Ok(())
    }

    pub fn add_transaction(&mut self, new_trns: transaction::Transaction) -> Result<(), Error> {
        let transaction_id = new_trns.generate_id();
        if self.transactions.get(&transaction_id).is_some() {
            return Err(Error::TransactionIdInUse(transaction_id));
        }

        for account_name in new_trns.get_associated_accounts() {
            if self.accounts.get(account_name).is_none() {
                return Err(Error::UnknownAccount(account_name.to_owned()));
            }
        }

        if !&self.get_transaction_balance(&new_trns).is_zero() {
            return Err(Error::UnbalancedTransaction)
        }

        for account_name in new_trns.get_associated_accounts() {
            self.accounts
                .get_mut(account_name)
                .unwrap()
                .add_transaction(transaction_id);
        }

        self.transactions.insert(transaction_id, new_trns);

        Ok(())
    }

    pub fn remove_transaction(&mut self, transaction_id: transaction::TransactionId) -> Result<(), Error> {
        let Some(transaction) = self.transactions.get(&transaction_id) else {
            return Err(Error::UnknownTransaction(transaction_id))
        };

        for account_name in transaction.get_associated_accounts() {
            let account = self.accounts.get_mut(account_name).unwrap();
            account.remove_transaction(&transaction_id)
                .map_err(|_| Error::AccountNotAssociatedWithTransaction(
                        (account_name.to_owned(), transaction_id)
                    )
                )?;
        }
        Ok(())
    }

    pub fn get_account_balance(
        &self,
        account_name: &account::AccountName,
        start_date: Option<datetime::DateTime>,
        end_date: Option<datetime::DateTime>,
    ) -> Result<money::Amount, &'static str> {
        let total_amount: money::Amount = self.accounts
            .get(account_name)
            .ok_or("Account not found")?
            .get_transaction_ids()
            .map(|id| self.transactions.get(id).unwrap())
            .filter(|trns| {
                start_date.as_ref().map_or(true, |date| trns.get_date() >= date)
                &&
                end_date.as_ref().map_or(true, |date| trns.get_date() <= date)
            })
            .map(|trns: &transaction::Transaction| {
                let output: &money::Amount = trns.get_amount(account_name).unwrap();
                output
            }).fold(money::Amount::default(), |acc, x| acc + x);

        Ok(total_amount)
    }

    pub fn get_transaction_balance(&self, transaction: &transaction::Transaction) -> money::Amount {
        let mut total_balance = money::Amount::default();
        for (account_name, amount) in transaction.get_amounts() {
            match self.accounts.get(account_name).unwrap().get_account_type() {
                account::AccountType::Asset => {
                    total_balance = total_balance + amount;
                },
                account::AccountType::Flow => {
                    total_balance = total_balance - amount;
                }
            }
        }
        total_balance
    }
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
        for (trns_id, transaction) in self.transactions.iter() {
            for acc_name in transaction.get_associated_accounts() {
                self.accounts
                    .get_mut(acc_name)
                    .unwrap()
                    .add_transaction(*trns_id);
            }
        }

        Ok(())
    }
}

fn serialize_transactions<S: Serializer>(
    map: &HashMap<transaction::TransactionId, transaction::Transaction>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.collect_seq(map.iter().map(|(_, x)| x))
}

fn deserialize_transactions<'de, D>(
    deserializer: D,
) -> Result<HashMap<transaction::TransactionId, transaction::Transaction>, D::Error>
where
    D: Deserializer<'de>,
{
    struct SeqVisitor;

    impl<'de> Visitor<'de> for SeqVisitor {
        type Value = HashMap<transaction::TransactionId, transaction::Transaction>;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Whatttt")
        }

        fn visit_seq<S>(self, mut sequence: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            let mut map = HashMap::new();

            while let Some(trns) = sequence.next_element::<transaction::Transaction>()? {
                map.insert(trns.generate_id(), trns);
            }
            Ok(map)
        }
    }

    deserializer.deserialize_seq(SeqVisitor)
}

fn serialize_accounts<S: Serializer>(
    map: &HashMap<account::AccountName, account::Account>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.collect_seq(map.iter().map(|(_, x)| x))
}

fn deserialize_accounts<'de, D>(
    deserializer: D,
) -> Result<HashMap<account::AccountName, account::Account>, D::Error>
where
    D: Deserializer<'de>,
{
    struct SeqVisitor;

    impl<'de> Visitor<'de> for SeqVisitor {
        type Value = HashMap<account::AccountName, account::Account>;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Whatttt")
        }

        fn visit_seq<S>(self, mut sequence: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            let mut map = HashMap::new();

            while let Some(account) = sequence.next_element::<account::Account>()? {
                map.insert(account.get_name().to_owned(), account);
            }
            Ok(map)
        }
    }

    deserializer.deserialize_seq(SeqVisitor)
}
