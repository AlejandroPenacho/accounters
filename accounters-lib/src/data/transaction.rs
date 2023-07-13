use crate::data::datetime::DateTime;
use serde::{Deserialize, Serialize};

use crate::data::account::AccountName;

use std::hash::{Hash, Hasher};

#[derive(Deserialize, Serialize, Hash)]
pub struct Transaction {
    id: TransactionId,
    name: String,
    notes: String,
    datetime: DateTime,
    accounts: Vec<TransactionMovement>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct TransactionId(u64);

#[derive(Deserialize, Serialize, Hash)]
pub struct TransactionMovement {
    account: AccountName,
    amount: Amount,
    currency: Currency,
}

#[derive(Deserialize, Serialize, Hash)]
pub struct Currency(String);

#[derive(Deserialize, Serialize, Hash)]
pub struct Amount {
    units: u64,
    subs: u64,
    sub_factor: u64,
}

impl Transaction {
    pub fn example_transaction(
        name: &str,
        notes: &str,
        datetime: DateTime,
        accounts: &[(&str, u64)],
    ) -> Transaction {
        let mut transaction = Transaction {
            id: TransactionId::default(),
            name: name.to_owned(),
            notes: notes.to_owned(),
            datetime,
            accounts: accounts
                .iter()
                .map(|(acc_name, amount)| TransactionMovement {
                    account: AccountName::new(acc_name),
                    amount: Amount {
                        units: *amount,
                        subs: 0,
                        sub_factor: 0,
                    },
                    currency: Currency("EUR".to_owned()),
                })
                .collect(),
        };

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        transaction.hash(&mut hasher);

        transaction.id = TransactionId(hasher.finish());

        transaction
    }

    pub fn get_id(&self) -> TransactionId {
        self.id
    }

    pub fn get_associated_accounts(&self) -> impl Iterator<Item = &AccountName> {
        self.accounts.iter().map(|x| &x.account)
    }
}
