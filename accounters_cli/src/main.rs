mod db_loader;

use accounters_lib::data::{
    Database,
    transaction::TransactionId
};

fn main() {
    let (name, mut database) = db_loader::load_database("files").unwrap();
    let state = State::init(name, &mut database);
    state.print();
}

enum Input {
    Literal(String),
    Integer(i64),
    Float(f64)
}

struct State<'a> {
    database: &'a mut Database,
    db_name: String,
    mode: Mode
}

enum Mode {
    StartScreen,
    TransactionView(TransactionViewState),
    TransactionViewConfiguration(TransactionViewConfig)
}

struct TransactionViewState {
    id_list: Vec<TransactionId>,
    current_range: (usize, usize),
    config: TransactionViewConfig
}
struct TransactionViewConfig {
    filter_account: Option<String>
}

impl<'a> State<'a> {
    fn init(db_name: String, database: &'a mut Database) -> Self {
        State {
            db_name,
            database,
            mode: Mode::StartScreen
        }
    }

    fn print(&self) {
        use Mode::*;
        let n_lines = termsize::get().unwrap().rows as usize;
        match &self.mode {
            StartScreen => {
                println!("Loaded database {}\n\nSelect action:", self.db_name);
                println!();
                println!("\t1) Show transactions");
                println!("\t2) Delete database");
                println!("\tq) Exit");
                println!("{}", "\n".repeat(n_lines - 10));
                println!("Press index or q:");
            },
            TransactionView(tv_state) => {
                println!("Nothing");
            },
            TransactionViewConfiguration(config) => {
             println!("More nothing");
            }
        }
    }

    fn read(&self) -> Input {
        Input::Integer(32)
    }

    fn eval(&self, input: Input) -> Option<State> {
        None
    }
}
