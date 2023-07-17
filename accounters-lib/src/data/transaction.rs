use crate::data::datetime::DateTime;
use serde::{Deserialize, Serialize};

use crate::data::account::AccountName;

use std::hash::{Hash, Hasher};

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
    currency: Currency,
}

#[derive(Deserialize, Serialize, Hash)]
pub struct Currency(String);

#[derive(Deserialize, Serialize, Hash)]
pub struct Amount {
    units: i64,
    subs: i64,
    sub_factor: i64,
}

impl std::ops::Add for Amount {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut units = self.units + other.units;

        let sub_factor = self.sub_factor.max(other.sub_factor);

        Amount {
            units,
            subs: 0,
            sub_factor,
        }
    }
}

impl Transaction {
    pub fn example_transaction(
        name: &str,
        notes: &str,
        datetime: DateTime,
        accounts: &[(&str, i64)],
    ) -> Transaction {
        Transaction {
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
