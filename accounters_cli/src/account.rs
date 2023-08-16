use accounters_lib::data::{
    Database,
    account::AccountType
};
use std::fmt::Write;

pub struct MultiAccountViewState {
    account_type: AccountType
}

impl MultiAccountViewState {
    pub fn new() -> Self {
        Self { account_type: AccountType::Asset }
    }

    pub fn produce_text(&self, database: &Database) -> String { 
        let mut output = String::from("\n\n\n");

        let accounts = database.get_account_names().map(|name| {
                (name, database.get_account(name))
            }).filter_map(|(name, acc)| {
                if acc.get_account_type() != &self.account_type {
                    None
                } else {
                    Some((
                        name, 
                        database.get_account_balance(name, None, None).unwrap()
                    ))
                }
            }).collect::<Vec<_>>();

        /*
            writeln!(
                output, 
                "{:>35}  : {}", account_name.as_ref(), balance
            ).unwrap();
        }
        */
        output
    }
}
