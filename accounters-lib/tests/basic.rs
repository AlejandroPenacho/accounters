use accounters_lib::data::{
    account::{Account, AccountType, AccountName},
    datetime::DateTime,
    transaction::Transaction,
    money::Amount,
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
                ("bank/ICA_Bank", "-132 SEK"),
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


    let removal_try = database_2.remove_account(AccountName::new("shares/monopoly"));
    assert!(matches!(
            removal_try,
            Err(accounters_lib::data::Error::AccountHasTransactions(_))
    ));

    database_2.save_to_file("test_files/file_1.txt");
}

#[test]
fn compute_balance() {
    let mut database = Database::default();
    database.add_account(Account::new("bank/ICA_Bank", AccountType::Asset)).unwrap();
    database.add_account(Account::new("balance/splitwise", AccountType::Asset)).unwrap();
    database.add_account(Account::new("bank/BBVA", AccountType::Asset)).unwrap();

    database.add_transaction(Transaction::example_transaction(
        "cosas",
        "Nada",
        DateTime::from_str("2023-8-16").unwrap(),
        &[
            ("bank/ICA_Bank", "2500 SEK"),
            ("balance/splitwise", "-2500 SEK"),
        ]
    )).unwrap();

    database.add_transaction(Transaction::example_transaction(
        "Devolucion",
        "Habia que",
        DateTime::from_str("2023-8-23").unwrap(),
        &[
            ("bank/ICA_Bank", "-1300 SEK"),
            ("balance/splitwise", "1300 SEK"),
        ]
    )).unwrap();

    database.add_transaction(Transaction::example_transaction(
        "Otra",
        "Habia que",
        DateTime::from_str("2023-9-03").unwrap(),
        &[
            ("bank/BBVA", "-800 EUR"),
            ("balance/splitwise", "800 EUR"),
        ]
    )).unwrap();

    assert_eq!(
        Amount::from_str("800 EUR, -1200 SEK"),
        database.get_account_balance(&AccountName::new("balance/splitwise"), None, None)
    );

    assert_eq!(
        Amount::from_str("-1200 SEK"),
        database.get_account_balance(&AccountName::new("balance/splitwise"), None, Some(DateTime::from_str("2023-8-31").unwrap()))
    );
}
