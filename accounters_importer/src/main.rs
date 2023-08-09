mod importer;

use accounters_lib::data::{
    Database,
    account::{
        Account,
        AccountType
    }
};

use std::collections::HashSet;

fn main() {
    let database = import_database("files/blue_trns.csv");
    database.save_to_file("files/blue_database.json");
}

fn import_database(path: &str) -> Database {
    let mut database = Database::default();

    let transactions = importer::import_transactions(path).unwrap();

    let mut account_names = HashSet::new();

    for transaction in transactions.iter() {
        for account_name in transaction.get_associated_accounts() {
            account_names.insert(account_name);
        }
    }

    for account_name in account_names {
        let account_class = match account_name.as_ref().split_once('/').unwrap().0 {
            "asset" => AccountType::Asset,
            _ => AccountType::Flow
        };
        database.add_account(Account::new(account_name.as_ref(), account_class)).unwrap();
    }

    for transaction in transactions {
        database.add_transaction(transaction).unwrap();
    }

    database
}
