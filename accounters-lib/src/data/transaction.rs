use crate::data::datetime::DateTime;
use serde::{Deserialize, Serialize};

use crate::data::{account::AccountName, money::Amount};

use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Deserialize, Serialize, Hash)]
pub struct Transaction {
    name: String,
    notes: String,
    datetime: DateTime,
    accounts: Vec<TransactionMovement>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Default, Debug)]
pub struct TransactionId(u64);

#[derive(Deserialize, Serialize, Hash)]
pub struct TransactionMovement {
    account: AccountName,
    amount: Amount,
}

impl Transaction {
    pub fn example_transaction(
        name: &str,
        notes: &str,
        datetime: DateTime,
        accounts: &[(&str, &str)],
    ) -> Transaction {
        Transaction {
            name: name.to_owned(),
            notes: notes.to_owned(),
            datetime,
            accounts: accounts
                .iter()
                .map(|(acc_name, amount)| TransactionMovement {
                    account: AccountName::new(acc_name),
                    amount: Amount::from_str(amount).unwrap(),
                })
                .collect(),
        }
    }

    pub fn get_associated_accounts(&self) -> impl Iterator<Item = &AccountName> {
        self.accounts.iter().map(|x| &x.account)
    }

    pub fn get_movements(&self) -> impl Iterator<Item = &TransactionMovement> {
        self.accounts.iter()
    }

    pub fn generate_id(&self) -> TransactionId {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        TransactionId(hasher.finish())
    }
}
