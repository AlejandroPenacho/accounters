use serde::{Deserialize, Serialize};

use std::{
    str::FromStr,
    collections::HashMap,
    fmt::Write
};

const SEP_THOUSAND: &str = ",";
const SEP_DEC: &str = ".";

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default, Clone)]
pub struct Amount {
    amounts: HashMap<Currency, Number>
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Default, Hash, Clone)]
pub struct Currency(pub String);

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

    pub fn currencies(&self) -> Vec<Currency> {
        self.amounts.keys().cloned().collect()
    }

    pub fn in_currency(&self, currency: &Currency) -> Number {
        self.amounts.get(currency).map_or(Number::default(), |x| x.clone())
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

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for (currency, number) in self.amounts.iter() {
            write!(output, "{} {}, ", number, currency.0)?;
        }
        output.fmt(f)
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

        let sign = if units >= 0 { 1 } else { -1};

        let decimals_string = split.next().ok_or("What!")?.trim_end_matches('0');

        let n_decimals = decimals_string.len() as u32;

        let mut value = units * 10i64.pow(n_decimals);
        let decimals = if n_decimals == 0 {
            0
        } else {
            decimals_string.parse::<i64>().map_err(|_| "Unparsable")? * sign
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

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (sign, value, decimals) = self.get_strings();

        let output = format!(
            "{}{:0<1}{}{:0<2}",
            sign,
            value,
            SEP_DEC,
            decimals
        );
        
        output.fmt(f)
    }
}

impl Number {
    pub fn is_zero(&self) -> bool {
        self.value == 0
    }
    pub fn is_nonnegative(&self) -> bool {
        self.value >= 0
    }

    /// I hope this makes everything nicer, because so far it has been quite
    /// embarrasing
    pub fn get_strings(&self) -> (String, String, String) {
        let str_value = self.value.abs().to_string();
        let n_decimals = self.n_decimals as usize;

        let (naive_units, decimals) = if str_value.len() > n_decimals {
            let borrowed = str_value.split_at(str_value.len() - n_decimals);
            (
                borrowed.0.to_owned(),
                borrowed.1.to_owned(),
            )
        } else {
            // println!("Se vino: {}", self);
            (
                String::new(),
                format!("{:0>1$}", str_value, n_decimals)
            )
        };

        let sign = if self.is_nonnegative() { "" } else { "-" };

        let mut reversed_units = String::new();

        for (i, c) in naive_units.chars().rev().enumerate() {
            let has_comma = i % 3 == 0 && i != 0 && c != '-';
            if has_comma {
                reversed_units.push(',')
            }
            reversed_units.push(c);
        }

        (
            sign.to_owned(), 
            reversed_units.chars().rev().collect(),
            decimals.to_owned()
        )
    }

    pub fn format(&self, alignment: &NumberAlignment) -> String {
        let (mut sign, units, mut decimals) = self.get_strings();

        if sign.is_empty() { sign = " ".to_string() };
        decimals = format!("{:0>2}", decimals);

        if alignment.minus_alignment {
            format!(
                "{}{:>3$}.{:<4$}",
                sign, units, decimals,
                alignment.unit_slots, alignment.decimal_slots
            )
        } else {
            format!(
                "{:>2$}.{:<3$}",
                format!("{}{}", sign, units), decimals,
                alignment.unit_slots + 1, alignment.decimal_slots
            )

        }
    }
}

pub struct NumberAlignment {
    minus_alignment: bool,
    unit_slots: usize,
    decimal_slots: usize,
}

impl NumberAlignment {
    pub fn from_numbers<T: IntoIterator<Item=Number>>(numbers: T) -> Self {
        let mut unit_slots: usize = 0;
        let mut decimal_slots: usize = 0;
        for number in numbers {
            let (_, units, decimals) = number.get_strings();
            let n_decimals = decimals.len().max(2);
            let n_units = units.len();
            unit_slots = unit_slots.max(n_units);
            decimal_slots = decimal_slots.max(n_decimals);
            println!("Number: {}, n_units: {}", number, n_units);
        }

        Self {
            minus_alignment: true,
            unit_slots,
            decimal_slots
        }
    }
}

/*
impl AmountAlignment {
    pub fn format_amount(&self, amount: &Amount) -> String {
        let mut output = String::new();
        for (currency, units_size, decimal_size) in self.0.iter() {
            let number = amount.in_currency(currency);
            let str_number = number.to_string();
            let (units, decimals) = str_number.split_once(',').expect("Something with number formatting");
            if number.is_zero() {
                write!(output, "{:<0$}", units_size + decimal_size + currency.0.len() + 2).unwrap();

            } else {
                write!(
                    output,
                    "{:>3$}.{:<4$} {}",
                    units,
                    decimals,
                    currency.0,
                    units_size,
                    decimal_size
                ).unwrap()
            }
        }
        output
    }
}
*/

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
            assert_eq!(
                Number{ value: 174534, n_decimals: 3 },
                Number::from_str("174.534").unwrap()
            );
            assert_eq!(
                Number{ value: 534, n_decimals: 3 },
                Number::from_str("0.534").unwrap()
            );
            assert_eq!(
                Number{ value: 4, n_decimals: 3 },
                Number::from_str("0.00400").unwrap()
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

        #[test]
        fn printing() {
            assert_eq!(
                (
                    String::from("-"),
                    String::from("123,456"),
                    String::from("425")
                ),
                Number::from_str("-123456.425").unwrap().get_strings()
            );
            assert_eq!(
                (
                    String::from(""),
                    String::from(""),
                    String::from("43")
                ),
                Number::from_str("0.43").unwrap().get_strings()
            );
            assert_eq!(
                (
                    String::from(""),
                    String::from(""),
                    String::from("0017")
                ),
                Number::from_str("0.0017").unwrap().get_strings()
            );
        }
    }
}
