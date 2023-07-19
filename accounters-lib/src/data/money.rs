use serde::{Deserialize, Serialize};

use std::str::FromStr;

#[derive(Debug)]
pub struct MultiCurrencyAmount {
    amounts: Vec<Amount>,
}

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, Debug)]
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
