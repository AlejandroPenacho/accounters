mod db_loader;
mod transaction;
mod account;

use accounters_lib::data::{
    Database,
    money::Amount,
    datetime::DateTime
};

use transaction::{
    TransactionViewState,
    MultiTransactionViewState,
    MultiTransactionViewConfig,
    TransactionEditState
};
use account::MultiAccountViewState;

fn main() {
    let (name, database) = db_loader::load_database("files").unwrap();
    let mut state = State::init(name, database);
    loop {
        state.print();
        let mut text_input = String::new();
        std::io::stdin().read_line(&mut text_input).unwrap();
        let input = state.read(text_input.trim());
        println!("Parsed: {:?}", input);
        state.eval(input);
        if state.mode.is_empty() {
            break
        }
    }
    let n_lines = termsize::get().unwrap().rows as usize;
    println!("Thank you, never come back!");
    println!("{}", "\n".repeat(n_lines-3));
}

#[derive(Debug)]
enum Input {
    Quit,
    Literal(String),
    Amount(Amount),
    Integer(i64),
    DateTime(DateTime)
}

struct State {
    database: Database,
    db_name: String,
    mode: Vec<Mode>
}

enum Mode {
    StartScreen,
    MultiTransactionView(MultiTransactionViewState),
    MultiTransactionViewConfiguration(MultiTransactionViewConfig),
    TransactionView(TransactionViewState),
    MultiAccountView(MultiAccountViewState),
    TransactionEdit(TransactionEditState)
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
                top_text.push_str("\t1) Show accounts\n");
                top_text.push_str("\t2) Show transactions\n");
                top_text.push_str("\t3) Delete database\n");
                top_text.push_str("\tq) Exit\n");
                let bottom_text = String::from("Press index or q:");
                (top_text, bottom_text)
            },
            MultiTransactionView(tv_state) => {
                let (top, bottom) = tv_state.produce_text(&self.database);
                (top, bottom)
            },
            MultiTransactionViewConfiguration(config) => {
                (String::from("a"), String::from("b"))
            },
            TransactionView(transaction_view) => {
                (
                    transaction_view.produce_text(&self.database),
                    String::from("Press anything, it is probably not going to work XDDD")
                )
            },
            MultiAccountView(view_state) => {
                (
                    view_state.produce_text(&self.database),
                    String::from("Show assets (a), flows (f), or go back (q)")
                )
            },
            TransactionEdit(te_state) => {
                (String::from("OMG\n"), String::from("Please go back (q)"))
            }
        };

        top_text.push_str(&"\n".repeat(n_lines - top_text.lines().count() - 2));
        println!("{}{}", top_text, bottom_text);
    }

    fn read(&self, input: &str) -> Input {
        if input == "q" || input == "quit" {
            return Input::Quit
        }
        if let Ok(datetime) = input.parse::<DateTime>() {
            return Input::DateTime(datetime)
        }
        if let Ok(amount) = input.parse::<Amount>() {
            return Input::Amount(amount)
        }
        if let Ok(integer) = input.parse::<i64>() {
            return Input::Integer(integer)
        }
        Input::Literal(input.to_owned())
    }

    fn eval(&mut self, input: Input) {
        if matches!(input, Input::Quit) {
            self.mode.pop();
            return
        }
        match self.mode.iter_mut().last().unwrap() {
            Mode::StartScreen => start_screen_select_mode(self, input),
            Mode::MultiTransactionView(tv_state) => {

                if let Input::Literal(input) = input {
                    if input == "f" {
                        tv_state.move_forward(None);
                    } else if input == "b" {
                        tv_state.move_back(None);
                    }
                    return
                }

                if let Input::Integer(index) = input {
                    let transaction_id = *tv_state.get_transaction_id(index as usize);
                    self.mode.push(Mode::TransactionView(TransactionViewState::new(transaction_id)));
                }
            },
            Mode::MultiAccountView(av_state) => {
                let Input::Literal(input) = input else {
                    return
                };
                if input == "f" {
                    av_state.show_flows()
                } else if input == "a" {
                    av_state.show_assets()
                }
            },
            _ => {}
        }
    }
}

fn start_screen_select_mode(state: &mut State, input: Input) {
    let Input::Integer(input) = input else {
        return
    };
    match input {
        1 => { state.mode.push(Mode::MultiAccountView(MultiAccountViewState::new())) },
        2 => { state.mode.push(Mode::MultiTransactionView(MultiTransactionViewState::new(&state.database))) },
        _ => { }
    }
}
