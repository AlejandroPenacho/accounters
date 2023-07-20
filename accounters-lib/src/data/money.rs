use serde::{Deserialize, Serialize};

use std::{
    str::FromStr,
    collections::HashMap
};

macro_rules! multi_amounts {
    ($($a:expr),+) => {{
        let mut amounts = HashMap::new();
        $(
            let c_amount = Amount::from_str($a).unwrap();
            amounts.insert(c_amount.currency.to_owned(), c_amount);
        )+
        MultiCurrencyAmount{ amounts }
    }};
}

#[derive(Debug, PartialEq, Eq)]
pub struct MultiCurrencyAmount {
    amounts: HashMap<String, Amount>
}

impl std::ops::Add for MultiCurrencyAmount {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        for other_amount in other.amounts.into_iter().map(|x| x.1) {
            if let Some(original_amount) = self.amounts.get_mut(&other_amount.currency) {
                let mut new_amount = std::mem::take(original_amount);
                new_amount = new_amount.add(other_amount).unwrap();
                *original_amount = new_amount;
            } else {
                self.amounts.insert(other_amount.currency.to_owned(), other_amount);
            }
        }

        self
    }
}

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, Debug, Default)]
pub struct Amount {
    value: i64,
    n_decimals: u32,
    currency: String,
}

impl FromStr for Amount {
    type Err = &'static str;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut first_split = str.split(' ');

        let quantity = first_split.next().ok_or("Come onn")?;
        let currency = first_split.next().ok_or("Nooo")?.to_owned();

        let mut split = if quantity.contains(',') {
            quantity.split(',')
        } else if quantity.contains('.') {
            quantity.split('.')
        } else {
            return Ok(Amount {
                value: quantity.parse().map_err(|_| "Unparsable")?,
                n_decimals: 0,
                currency,
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

        Ok(Amount {
            value,
            n_decimals,
            currency,
        })
    }
}

impl std::cmp::PartialOrd for Amount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.currency != other.currency {
            return None;
        }

        let max_decimals = self.n_decimals.max(other.n_decimals);

        Some(
            (self.value * 10i64.pow(max_decimals - self.n_decimals))
                .cmp(&(other.value * 10i64.pow(max_decimals - other.n_decimals))),
        )
    }
}

impl Amount {
    fn add(self, other: Amount) -> Result<Amount, ()> {
        if self.currency != other.currency {
            return Err(());
        }

        let n_decimals = self.n_decimals.max(other.n_decimals);

        let value = self.value * 10i64.pow(n_decimals - self.n_decimals)
            + other.value * 10i64.pow(n_decimals - other.n_decimals);

        Ok(Amount {
            value,
            n_decimals,
            currency: self.currency,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    mod multi_currency_amoung {
        use super::*;
        #[test]
        fn adding() {
            let first_amount = multi_amounts!(
                "42.1 EUR",
                "92 USD",
                "150.00 SEK"
            );
            let second_amount = multi_amounts!(
                "200.23 SEK",
                "-100 USD",
                "15 PLN"
            );

            let final_amount = first_amount + second_amount;

            assert_eq!(
                multi_amounts!("42.1 EUR", "-8 USD", "350.23 SEK", "15 PLN"),
                final_amount
            );
        }
    }
    mod amount {
        use super::*;
        #[test]
        fn parsing() {
            let amount = Amount::from_str("174.534 SEK");
            println!("{amount:?}");
        }

        #[test]
        fn addition() {
            let amount = Amount::from_str("154.32 SEK")
                .unwrap()
                .add(Amount::from_str("200.023 SEK").unwrap());
            assert_eq!(Amount::from_str("354.343 SEK").unwrap(), amount.unwrap());

            let amount = Amount::from_str("227 EUR")
                .unwrap()
                .add(Amount::from_str("-531.276 EUR").unwrap());
            assert_eq!(Amount::from_str("-304.276 EUR").unwrap(), amount.unwrap());
        }
    }
}
