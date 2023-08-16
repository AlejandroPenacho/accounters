use accounters_lib::data::{
    Database,
    account::AccountType,
    money::Currency
};
use std::{
    fmt::Write,
    collections::{
        HashSet,
        HashMap
    }
};

pub struct MultiAccountViewState {
    account_type: AccountType
}

impl MultiAccountViewState {
    pub fn new() -> Self {
        Self { account_type: AccountType::Asset }
    }

    pub fn show_assets(&mut self) {
        self.account_type = AccountType::Asset
    }

    pub fn show_flows(&mut self) {
        self.account_type = AccountType::Flow
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

        let mut currencies = HashSet::new();

        accounts.iter().flat_map(|(_, amount)| {
            amount.currencies().to_vec()
        }).for_each(|currency| { currencies.insert(currency); });

        let currencies = currencies.iter().collect::<Vec<_>>();

        let mut number_lengths: HashMap<Currency, usize> = HashMap::new();
        for currency in currencies.iter() {
            let max_length = accounts.iter().map(|(_, amount)| {
                amount.in_currency(currency).to_string().len()
            }).max().unwrap();
            number_lengths.insert((*currency).clone(), max_length);
        }

        let account_name_length = accounts.iter().map(|(name, _)| {
            name.as_ref().len()
        }).max().unwrap();


        for (account_name, amount) in accounts {
            write!(
                output,
                "{:>1$} :",
                account_name.as_ref(),
                account_name_length + 5
            ).unwrap();
            for currency in currencies.iter() {
                write!(
                    output,
                    " {:>1$}",
                    amount.in_currency(currency),
                    number_lengths.get(currency).unwrap() + 5
                ).unwrap();
            }
            writeln!(output).unwrap();
        }

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
