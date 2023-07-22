use serde::{Deserialize, Serialize};

use std::{
    str::FromStr,
    collections::HashMap
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Amount {
    amounts: HashMap<String, Number>
}

impl std::hash::Hash for Amount {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (currency, amount) in self.amounts.iter() {
            currency.hash(state);
            amount.hash(state);
        }
    }
}

impl FromStr for Amount {
    type Err= &'static str;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut output = HashMap::new();
        
        let splitted_input = input.split(", ");
        
        for element in splitted_input {
            let mut split = element.split(' ');
            let number = split.next().unwrap().parse().unwrap();
            let currency = split.next().unwrap().to_owned();

            output.insert(currency, number);
        }

        Ok(Amount { amounts: output })
    }
}

impl std::ops::Add<&Amount> for Amount {
    type Output = Self;
    fn add(mut self, other: &Self) -> Self {
        for (other_currency, other_amount) in other.amounts.iter() {
            if let Some(original_amount) = self.amounts.get_mut(other_currency) {
                let mut new_amount = std::mem::take(original_amount);
                new_amount = new_amount + other_amount.clone();
                *original_amount = new_amount;
            } else {
                self.amounts.insert(other_currency.to_owned(), other_amount.clone());
            }
        }

        self
    }
}

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, Debug, Default, Clone)]
pub struct Number {
    value: i64,
    n_decimals: u32,
}

impl FromStr for Number {
    type Err = &'static str;
    fn from_str(str: &str) -> Result<Self, Self::Err> {

        let mut split = if str.contains(',') {
            str.split(',')
        } else if str.contains('.') {
            str.split('.')
        } else {
            return Ok(Number {
                value: str.parse().map_err(|_| "Unparsable")?,
                n_decimals: 0,
            });
        };

        let units: i64 = split
            .next()
            .ok_or("What!")?
            .parse()
            .map_err(|_| "Unparsable")?;

        let decimals_string = split.next().ok_or("What!")?;

        let n_decimals = decimals_string.len() as u32;

        let mut value = units * 10i64.pow(n_decimals);
        let decimals = decimals_string.parse::<i64>().map_err(|_| "Unparsable")? * value.signum();

        value += decimals;

        Ok(Number {
            value,
            n_decimals,
        })
    }
}


impl std::ops::Add for Number {
    type Output = Number;
    fn add(self, other: Number) -> Self::Output {
        let n_decimals = self.n_decimals.max(other.n_decimals);

        let value = self.value * 10i64.pow(n_decimals - self.n_decimals)
            + other.value * 10i64.pow(n_decimals - other.n_decimals);

        Number {
            value,
            n_decimals,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    mod amount {
        use super::*;
        #[test]
        fn adding() {
            let first_amount = Amount::from_str("42.1 EUR, 92 USD, 150.00 SEK").unwrap();
            let second_amount = Amount::from_str("200.23 SEK, -100 USD, 15 PLN").unwrap();
            let final_amount = first_amount + &second_amount;

            assert_eq!(
                Amount::from_str("42.1 EUR, -8 USD, 350.23 SEK, 15 PLN").unwrap(),
                final_amount
            );
        }
    }
    mod number {
        use super::*;
        #[test]
        fn parsing() {
            let amount = Number::from_str("174.534").unwrap();
            assert_eq!(
                Number{ value: 174534, n_decimals: 3 },
                amount
            );
        }

        #[test]
        fn addition() {
            let amount = Number::from_str("154.32")
                .unwrap()
                +
                Number::from_str("200.023").unwrap();
            assert_eq!(Number::from_str("354.343").unwrap(), amount);

            let amount = Number::from_str("227")
                .unwrap()
                 + Number::from_str("-531.276").unwrap();
            assert_eq!(Number::from_str("-304.276").unwrap(), amount);
        }
    }
}
