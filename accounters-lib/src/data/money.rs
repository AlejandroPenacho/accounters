use serde::{Deserialize, Serialize};

use std::{
    str::FromStr,
    collections::HashMap
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default, Clone)]
pub struct Amount {
    amounts: HashMap<Currency, Number>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default, Hash, Clone)]
pub struct Currency(String);

impl Currency {
    pub fn new(name: &str) -> Self {
        Self(name.to_owned())
    }
}

impl From<&str> for Currency {
    fn from(origin: &str) -> Self {
        Self::new(origin)
    }
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
            let number: Number = split.next().ok_or("Format issue")?.parse().map_err(|_| "Parse error")?;
            if number.is_zero() { continue }

            let currency = split.next().ok_or("Format issue")?.into();

            if split.next().is_some() {
                return Err("What the hell???!")
            }

            let overwritten = output.insert(currency, number).is_some();
            if overwritten {
                return Err("Same currency introduced twice")
            }
        }

        Ok(Amount { amounts: output })
    }
}

impl Amount {
    pub fn is_zero(&self) -> bool {
        self.amounts.is_empty()
    }
}

impl std::ops::Add<&Amount> for Amount {
    type Output = Self;
    fn add(mut self, other: &Self) -> Self {
        let mut zeroed_currencies = Vec::new();

        for (other_currency, other_amount) in other.amounts.iter() {
            if let Some(saved_amount) = self.amounts.get_mut(other_currency) {
                let mut new_amount = std::mem::take(saved_amount);
                new_amount = new_amount + other_amount.clone();
                *saved_amount = new_amount;
                if saved_amount.is_zero() {
                    zeroed_currencies.push(other_currency);
                }
            } else {
                self.amounts.insert(other_currency.to_owned(), other_amount.clone());
            }
        }

        for currency in zeroed_currencies {
            self.amounts.remove(currency);
        }

        self
    }
}

impl std::ops::Sub<&Amount> for Amount {
    type Output = Self;
    fn sub(mut self, other: &Self) -> Self {
        self = self + &(-other);
        self
    }
}

impl std::ops::Neg for &Amount {
    type Output = Amount;
    fn neg(self) -> Self::Output {
        let mut output: Amount = self.to_owned();
        for (_, number) in output.amounts.iter_mut() {
            *number = -(number.clone());
        }
        output
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

        let decimals_string = split.next().ok_or("What!")?.trim_end_matches('0');

        let n_decimals = decimals_string.len() as u32;

        let mut value = units * 10i64.pow(n_decimals);
        let decimals = if n_decimals == 0 {
            0
        } else {
            decimals_string.parse::<i64>().map_err(|_| "Unparsable")? * value.signum()
        };

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
impl std::ops::Sub for Number {
    type Output = Number;
    fn sub(self, other: Number) -> Self::Output {
        self + (-other)
    }
}

impl std::ops::Neg for Number {
    type Output = Number;
    fn neg(mut self) -> Self {
        self.value *= -1;
        self
    }
}

impl Number {
    pub fn is_zero(&self) -> bool {
        self.value == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    mod amount {
        use super::*;
        #[test]
        fn parsing() {
            let first_amount = Amount::from_str("128.5430 EUR, -67.0004 PUM, 0.000 FOG");
            let second_amount = Amount::from_str("4 EUR 2 USD");
            let third_amount = Amount::from_str("45 EUR, 12 SEK, 11 EUR");

            println!("{:?}", second_amount);

            let mut amounts = HashMap::new();
            amounts.insert("EUR".into(), Number{ value: 128543, n_decimals: 3 });
            amounts.insert("PUM".into(), Number{ value: -670004, n_decimals: 4 });
            assert_eq!(
                Ok(Amount{ amounts }),
                first_amount
            );
            assert!(second_amount.is_err());
            assert!(third_amount.is_err());
        }

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

        #[test]
        fn zeroing() {
            let first_amount = Amount::from_str("243.3 EUR, 58 SEK").unwrap();
            let second_amount = Amount::from_str("-97.12 EUR, 23 SEK, 15 USD").unwrap();
            let third_amount = Amount::from_str("-146.18 EUR, -5 SEK").unwrap();

            let final_amount = first_amount + &second_amount + &third_amount;
            assert_eq!(
                Amount::from_str("15 USD, 76 SEK").unwrap(),
                final_amount
            );
        }

        #[test]
        fn substraction() {
            let first_amount = Amount::from_str("132 EUR, 34.2 USD, -43.2 SEK").unwrap();
            let second_amount = Amount::from_str("64 EUR, 34.2 USD, -85 SEK, 103 PLN").unwrap();
            let final_amount = first_amount - &second_amount;

            assert_eq!(
                Amount::from_str("68 EUR, 41.8 SEK, -103 PLN").unwrap(),
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
