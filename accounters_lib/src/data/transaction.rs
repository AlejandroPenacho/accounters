use crate::data::datetime::DateTime;
use serde::{Deserialize, Serialize};

use crate::data::{
    account::AccountName,
    money::Amount,
    tags::Tag
};

use std::hash::{Hash, Hasher};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Deserialize, Serialize)]
pub struct Transaction {
    name: String,
    notes: String,
    tags: HashSet<Tag>,
    datetime: DateTime,
    amounts: HashMap<AccountName, Amount>,
}

impl Hash for Transaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.notes.hash(state);
        self.datetime.hash(state);
        for tag in self.tags.iter() {
            tag.hash(state);
        }
        for (account, amount) in self.amounts.iter() {
            account.hash(state);
            amount.hash(state);
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Default, Debug)]
pub struct TransactionId(u64);

impl Transaction {
    pub fn example_transaction(
        name: &str,
        notes: &str,
        datetime: DateTime,
        amounts: &[(&str, &str)],
    ) -> Transaction {
        let mut amounts_map = HashMap::new();
        for (account, amount) in amounts {
            amounts_map.insert(AccountName::new(account), Amount::from_str(amount).unwrap());
        }

        Transaction {
            name: name.to_owned(),
            notes: notes.to_owned(),
            tags: HashSet::new(),
            datetime,
            amounts: amounts_map,
        }
    }

    pub fn from_amounts(
        name: &str,
        notes: &str,
        datetime: DateTime,
        amounts: &[(String, Amount)],
    ) -> Transaction {
        let mut amounts_map = HashMap::new();
        for (account, amount) in amounts {
            amounts_map.insert(AccountName::new(account), amount.to_owned());
        }

        Transaction {
            name: name.to_owned(),
            notes: notes.to_owned(),
            tags: HashSet::new(),
            datetime,
            amounts: amounts_map,
        }
    }

    pub fn get_associated_accounts(&self) -> impl Iterator<Item = &AccountName> {
        self.amounts.keys()
    }

    pub fn get_amount(&self, account_name: &AccountName) -> Result<&Amount, &'static str> {
        self.amounts.get(account_name).ok_or("Nope")
    }

    pub fn generate_id(&self) -> TransactionId {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        TransactionId(hasher.finish())
    }

    pub fn get_amounts(&self) -> &HashMap<AccountName, Amount> {
        &self.amounts
    }
    
    pub fn get_date(&self) -> &DateTime {
        &self.datetime
    }
}
