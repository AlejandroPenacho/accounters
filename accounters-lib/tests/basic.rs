use accounters_lib::data::{
    account::{Account, AccountType},
    datetime::DateTime,
    transaction::Transaction,
    Database,
};

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
            DateTime::simple((2023, 7, 13), Some((14, 54))),
            &[("bank/ICA_Bank", 132), ("entertainment/eat_out", 132)],
        ))
        .unwrap();

    database_1.save_to_file("test_files/output_file_1.txt");

    let mut database_2 = Database::read_from_file("test_files/input_file_1.txt").unwrap();

    database_2
        .add_account(Account::new("shares/monopoly", AccountType::Asset))
        .unwrap();

    database_2
        .add_transaction(Transaction::example_transaction(
            "Big sellot",
            "Hahahaha",
            DateTime::simple((2011, 7, 15), None),
            &[("shares/monopoly", 150), ("bank/ICA_Bank", -150)],
        ))
        .unwrap();

    database_2.save_to_file("test_files/output_file_1.txt");
}
