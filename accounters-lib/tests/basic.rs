use accounters_lib::data::{
    account::{Account, AccountType},
    datetime::DateTime,
    transaction::Transaction,
    Database,
};

use std::str::FromStr;

#[test]
fn read_and_write_file() {
    let mut database_1 = Database::default();

    database_1
        .add_account(Account::new("bank/ICA_Bank", AccountType::Asset))
        .unwrap();
    database_1
        .add_account(Account::new("entertainment/eat_out", AccountType::Flow))
        .unwrap();

    database_1
        .add_transaction(Transaction::example_transaction(
            "Comprar nabos",
            "Na que comentar xd",
            DateTime::from_str("2023-07-13 14:54").unwrap(),
            &[
                ("bank/ICA_Bank", "132 SEK"),
                ("entertainment/eat_out", "-132 SEK"),
            ],
        ))
        .unwrap();

    database_1.save_to_file("test_files/file_1.txt");

    let mut database_2 = Database::read_from_file("test_files/file_1.txt").unwrap();

    database_2
        .add_account(Account::new("shares/monopoly", AccountType::Asset))
        .unwrap();

    database_2
        .add_transaction(Transaction::example_transaction(
            "Big sellot",
            "Hahahaha",
            DateTime::from_str("2011-07-15").unwrap(),
            &[
                ("shares/monopoly", "150 EUR"),
                ("bank/ICA_Bank", "-150 EUR"),
            ],
        ))
        .unwrap();

    database_2.save_to_file("test_files/file_1.txt");
}
