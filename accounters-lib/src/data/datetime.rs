use serde::{Deserialize, Serialize};

use std::{cmp, ops, str::FromStr};

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime {
    date: Date,
    time: Option<Time>,
}

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    hour: u8,
    minute: u8,
}

impl DateTime {
    pub fn simple(date: (u16, u8, u8), time: Option<(u8, u8)>) -> Self {
        DateTime {
            date: Date {
                year: date.0,
                month: date.1,
                day: date.2,
            },
            time: time.map(|(h, m)| Time { hour: h, minute: m }),
        }
    }
}

impl FromStr for Date {
    type Err = &'static str;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        // It should look like YYYY-MM-DD

        let mut split = str.split('-');
        let year = split
            .next()
            .ok_or("What")?
            .parse()
            .map_err(|_| "Unparsable")?;
        let month = split
            .next()
            .ok_or("What")?
            .parse()
            .map_err(|_| "Unparsable")?;
        let day = split
            .next()
            .ok_or("What")?
            .parse()
            .map_err(|_| "Unparsable")?;
        Ok(Self { year, month, day })
    }
}

impl FromStr for Time {
    type Err = &'static str;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        // It should look like YYYY-MM-DD

        let mut split = str.split(':');
        let hour = split
            .next()
            .ok_or("What")?
            .parse()
            .map_err(|_| "Unparsable")?;

        let minute = split
            .next()
            .ok_or("What")?
            .parse()
            .map_err(|_| "Unparsable")?;

        Ok(Self { hour, minute })
    }
}

impl FromStr for DateTime {
    type Err = &'static str;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        // It should look like 2023-08-15 15:23
        // or
        // It should look like 2023-08-15
        // or
        // It should look like 2023-08-15T15:23

        let mut split = if str.contains('T') {
            str.split('T')
        } else if str.contains(' ') {
            str.split(' ')
        } else {
            return Ok(DateTime {
                date: str.parse()?,
                time: None,
            });
        };

        Ok(DateTime {
            date: split.next().ok_or("What")?.parse()?,
            time: Some(split.next().ok_or("What")?.parse()?),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod compare {
        use super::*;
        #[test]
        fn compare_time() {
            assert!(Time::from_str("12:56") < Time::from_str("12:57"));
            assert!(Time::from_str("6:52") < Time::from_str("7:04"));
        }

        #[test]
        fn compare_date() {
            assert!(Date::from_str("2023-11-08") < Date::from_str("2023-11-09"))
        }

        #[test]
        fn compare_datetime() {
            assert!(DateTime::from_str("2008-08-04") < DateTime::from_str("2008-08-04T00:05"))
        }
    }
}
