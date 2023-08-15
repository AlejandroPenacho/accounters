mod db_loader;
mod transaction;

use accounters_lib::data::{
    Database,
};

use transaction::{
    SingleTransactionViewState,
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
        state.eval(input);
        if state.mode.is_empty() {
            break
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
    mode: Vec<Mode>
}

enum Mode {
    StartScreen,
    TransactionView(TransactionViewState),
    TransactionViewConfiguration(TransactionViewConfig),
    SingleTransactionView(SingleTransactionViewState)
}


impl State {
    fn init(db_name: String, database: Database) -> Self {
        State {
            db_name,
            database,
            mode: vec![Mode::StartScreen]
        }
    }

    fn print(&self) {
        use Mode::*;
        let n_lines = termsize::get().unwrap().rows as usize;
        let (mut top_text, bottom_text) = match &self.mode.last().unwrap() {
            StartScreen => {
                let mut top_text = String::new();
                top_text.push_str(&format!("Loaded database {}\n\nSelect action:\n\n\n", self.db_name));
                top_text.push_str("\t1) Show transactions\n");
                top_text.push_str("\t2) Delete database\n");
                top_text.push_str("\tq) Exit\n");
                let bottom_text = String::from("Press index or q:");
                (top_text, bottom_text)
            },
            TransactionView(tv_state) => {
                let (top, bottom) = tv_state.produce_text(&self.database);
                (top, bottom)
            },
            TransactionViewConfiguration(config) => {
                (String::from("a"), String::from("b"))
            },
            SingleTransactionView(transaction_view) => {
                (
                    transaction_view.produce_text(&self.database),
                    String::from("Press anything, it is probably not going to work XDDD")
                )
            }
        };

        top_text.push_str(&"\n".repeat(n_lines - top_text.lines().count() - 2));
        println!("{}{}", top_text, bottom_text);
    }

    fn read(&self, input: &str) -> Input {
        if input == "q" || input == "quit" {
            return Input::Quit
        }
        Input::Literal(input.to_owned())
    }

    fn eval(&mut self, input: Input) {
        if matches!(input, Input::Quit) {
            self.mode.pop();
            return
        }
        match self.mode.iter_mut().last().unwrap() {
            Mode::StartScreen => start_screen_select_mode(self),
            Mode::TransactionView(tv_state) => {
                let Input::Literal(input) = input else {
                    return
                };
                if input == "f" {
                    tv_state.move_forward(None);
                } else if input == "b" {
                    tv_state.move_back(None);
                } else {
                    let input = input.parse::<usize>().unwrap();
                    let transaction_id = *tv_state.get_transaction_id(input);
                    self.mode.push(Mode::SingleTransactionView(SingleTransactionViewState::new(transaction_id)));
                }
            }
            _ => {}
        }
    }
}

fn start_screen_select_mode(state: &mut State) {
    let transaction_view_state = TransactionViewState::new(&state.database);
    state.mode.push(Mode::TransactionView(transaction_view_state));
}
