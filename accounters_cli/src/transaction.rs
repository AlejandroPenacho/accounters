use accounters_lib::data::{
    Database,
    transaction::TransactionId,
    datetime::{
        Date,
        Time,
        DateTime
    }
};

pub struct TransactionViewState {
    id_list: Vec<TransactionId>,
    current_range: (usize, usize),
    config: TransactionViewConfig
}

pub struct SingleTransactionViewState {
    transaction_id: TransactionId
}

pub struct TransactionViewConfig;

impl TransactionViewConfig {
    pub fn get_transactions_per_page(&self) -> usize {
        termsize::get().unwrap().rows as usize - 5
    }
}

impl TransactionViewState {
    pub fn new(database: &Database) -> Self {
        let mut all_transactions_id: Vec<TransactionId> = database.get_transaction_ids().cloned().collect();

        all_transactions_id.sort_by_key(|id| database.get_transaction(id).get_datetime());
        all_transactions_id.reverse();
        let config = TransactionViewConfig;

        Self {
            id_list: all_transactions_id,
            current_range: (0, config.get_transactions_per_page()),
            config
        }
    }

    pub fn get_transaction_id(&self, input: usize) -> &TransactionId {
        &self.id_list[self.current_range.0 + input - 1]
    }

    pub fn produce_text(&self, database: &Database) -> (String, String) {
        let mut last_date: Option<Date> = None;
        let mut output = format!(
            "Displaying transactions {}-{} out of {}\n\n",
            self.current_range.0 + 1,
            self.current_range.1 + 1,
            self.id_list.len()
        );
        for (index, transaction_index) in ((self.current_range.0)..(self.current_range.1)).enumerate() {
            let transaction = database.get_transaction(&self.id_list[transaction_index]);

            let date = transaction.get_datetime().get_date();
            let time = transaction.get_datetime().get_time();

            output.push_str(&format!(
                "\t{}\t{}  {} \t{}\n",
                index+1,
                if last_date.map_or(true, |x| x != *date) { format!("{}", date) } else { "          ".to_string() },
                time.map_or("     ".to_string(), |x| format!("{}", x)),
                transaction.get_name()
            ));

            last_date = Some(date.to_owned());
        }
        (output, String::from("Your move"))
    }

    pub fn move_forward(&mut self, _n: Option<usize>) {
        self.current_range = (
            self.current_range.1,
            self.current_range.1 + self.config.get_transactions_per_page()
        )
    }

    pub fn move_back(&mut self, _n: Option<usize>) {
        self.current_range = (
            self.current_range.0 - self.config.get_transactions_per_page(),
            self.current_range.0
        )
    }
}


impl SingleTransactionViewState {
    pub fn new(transaction_id: TransactionId) -> Self {
        Self { transaction_id }
    }
    pub fn produce_text(&self, database: &Database) -> String {
        let transaction = database.get_transaction(&self.transaction_id);
        let mut output = format!(
            "Transaction with id {}\n\n",
            self.transaction_id.0
        );
        output.push_str(&format!(
            "{:>30} : {}\n", "Name", transaction.get_name()
        ));
        output.push_str(&format!(
            "{:>30} : {}\n", "Date", transaction.get_datetime().get_date()
        ));
        output.push_str(&format!(
            "{:>30} :\n\n", "Amounts"
        ));
        for account in transaction.get_associated_accounts() {
            let amount = transaction.get_amount(account).unwrap();
            output.push_str(&format!(
                "{:>30} : {}\n",
                account.as_ref(),
                amount
            ));
        }

        output
    }
}
