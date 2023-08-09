use accounters_lib::data::{
    transaction::Transaction,
    datetime::DateTime,
    money::Amount
};
use std::fs::read_to_string;

pub fn import_transactions(path: &str) -> Result<Vec<Transaction>, &'static str> {
    let file = read_to_string(path).map_err(|_| "File does not exist")?;

    let lines = file.lines();
    let mut output = Vec::new();

    for line in lines.skip(1) {
        // println!("{}", line);
        let transaction = parse_line(line);
        output.push(transaction);

    }


    Ok(output)
}

fn parse_line(text: &str) -> Transaction {
    let mut elements = text.split('"').skip(1).step_by(2);

    let class = elements.next().unwrap();
    let date = elements.next().unwrap();
    let _time = elements.next().unwrap();
    let title = elements.next().unwrap();
    let amount = elements.next().unwrap();
    let currency = elements.next().unwrap();
    let _exchange_rate = elements.next().unwrap();
    let category_group = elements.next().unwrap();
    let category = elements.next().unwrap();
    let account = elements.next().unwrap();
    let notes = elements.next().unwrap();
    let _tags = elements.next().unwrap();
    let _state = elements.next().unwrap();


    let amount = format!("{} {}", amount, currency).parse::<Amount>().unwrap();

    let account_balances = if class == "Gastos" {
        [
            (
                format!("asset/{}", account),
                amount.to_owned()
            ),
            (
                format!("expense/{}/{}", category_group, category),
                amount
            )
        ]
    } else if class == "Ingresos" {
        [
            (
                format!("asset/{}", account),
                amount.to_owned()
            ),
            (
                format!("income/{}/{}", category_group, category),
                amount
            )
        ]
    } else {
        [
            (
                format!("asset/{}", account),
                amount.to_owned()
            ),
            (
                String::from("asset/transfer"),
                -&amount
            )
        ]
    };


    let datetime = date.rsplit_once(':').unwrap().0.parse::<DateTime>().unwrap();


    Transaction::from_amounts(
        title,
        notes,
        datetime,
        &account_balances
    )
}
