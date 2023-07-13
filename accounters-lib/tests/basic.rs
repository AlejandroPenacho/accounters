use accounters_lib::data::{
    account::{Account, AccountType},
    datetime::DateTime,
    transaction::Transaction,
    Database,
};

#[test]
fn create_file() {
    let mut database = Database::default();

    database
        .add_account(Account::new("bank/ICA_Bank", AccountType::Asset))
        .unwrap();
    database
        .add_account(Account::new("entertainment/eat_out", AccountType::Flow))
        .unwrap();

    database
        .add_transaction(Transaction::example_transaction(
            "Comprar nabos",
            "Na que comentar xd",
            DateTime::simple((2023, 7, 13), Some((14, 54))),
            &[("bank/ICA_Bank", 132), ("entertainment/eat_out", 132)],
        ))
        .unwrap();

    database.save_to_file("test_files/first_test.txt");
}
