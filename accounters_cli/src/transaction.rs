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

pub struct TransactionViewConfig;

impl TransactionViewConfig {
    pub fn get_transactions_per_page(&self) -> usize {
        30
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

    pub fn print(&self, database: &Database) {
        let mut last_date: Option<Date> = None;
        for index in (self.current_range.0)..(self.current_range.1) {
            let transaction = database.get_transaction(&self.id_list[index]);

            let date = transaction.get_datetime().get_date();
            let time = transaction.get_datetime().get_time();

            println!(
                "\t{}  {} \t{}",
                if last_date.map_or(true, |x| x != *date) { format!("{}", date) } else { "          ".to_string() },
                time.map_or("     ".to_string(), |x| format!("{}", x)),
                transaction.get_name()
            );

            last_date = Some(date.to_owned());
        }
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


