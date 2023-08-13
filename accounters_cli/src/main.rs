mod db_loader;
mod transaction;

use accounters_lib::data::{
    Database,
};

use transaction::{
    TransactionViewState,
    TransactionViewConfig
};

fn main() {
    let (name, database) = db_loader::load_database("files").unwrap();
    let mut state = State::init(name, database);
    loop {
        state.print();
        let mut text_input = String::new();
        std::io::stdin().read_line(&mut text_input).unwrap();
        let input = state.read(text_input.trim());
        match state.eval(input) {
            Some(new_state) => { state = new_state },
            None => break
        }
    }
    let n_lines = termsize::get().unwrap().rows as usize;
    println!("Thank you, never come back!");
    println!("{}", "\n".repeat(n_lines-3));
}

enum Input {
    Quit,
    Literal(String),
    Integer(i64),
    Float(f64)
}

struct State {
    database: Database,
    db_name: String,
    mode: Mode
}

enum Mode {
    StartScreen,
    TransactionView(TransactionViewState),
    TransactionViewConfiguration(TransactionViewConfig)
}


impl State {
    fn init(db_name: String, database: Database) -> Self {
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
                tv_state.print(&self.database);
            },
            TransactionViewConfiguration(config) => {
             println!("More nothing");
            }
        }
    }

    fn read(&self, input: &str) -> Input {
        if input == "q" || input == "quit" {
            return Input::Quit
        }
        Input::Literal(input.to_owned())
    }

    fn eval(mut self, input: Input) -> Option<State> {
        if matches!(input, Input::Quit) {
            return None
        }
        match &mut self.mode {
            Mode::StartScreen => Some(start_screen_select_mode(self)),
            Mode::TransactionView(tv_state) => {
                let Input::Literal(input) = input else {
                    return Some(self)
                };
                if input == "f" {
                    tv_state.move_forward(None);
                } else if input == "b" {
                    tv_state.move_back(None);
                };
                Some(self)
            }
            _ => Some(self)
        }
    }
}

fn start_screen_select_mode(mut state: State) -> State {
    let transaction_view_state = TransactionViewState::new(&state.database);
    state.mode = Mode::TransactionView(transaction_view_state);
    state
}
