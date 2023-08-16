use serde::{Deserialize, Serialize};

use std::str::FromStr;

const MONTH_DAYS: [u8; 13] = [
    0,
    31, 28, 31, 30, 31, 30,
    31, 31, 30, 31, 30, 31
];

fn year_is_leap(year: u16) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct DateTime {
    date: Date,
    time: Option<Time>,
}

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
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

    pub fn get_date(&self) -> &Date {
        &self.date
    }

    pub fn get_time(&self) -> &Option<Time> {
        &self.time
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

impl Date {
    fn next_day(mut self) -> Self {
        self.day += 1;
        let month_days = if self.month == 2 && year_is_leap(self.year) {
            29
        } else {
            MONTH_DAYS[self.month as usize]
        };
        if self.day > month_days {
            self.day = 1;
            self.month += 1;
            if self.month == 13 {
                self.month = 1;
                self.year += 1;
            }
        }
        self
    }
    fn prev_day(mut self) -> Self {
        self.day -= 1;
        if self.day == 0 {
            self.month -= 1;
            if self.month == 0 {
                self.month = 12;
                self.year -= 1;
            }
            self.day = if self.month == 2 && year_is_leap(self.year) {
                29
            } else {
                MONTH_DAYS[self.month as usize]
            }
        }
        self
    }
}

impl std::ops::Add<i64> for Date {
    type Output = Date;
    fn add(mut self, days: i64) -> Self::Output {
        if days >= 0 {
            for _ in 0..days {
                self = self.next_day();
            }
        } else {
            for _ in 0..(days.abs()) {
                self = self.prev_day();
            }
        }
        self
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:4}-{:0>2}-{:0>2}", self.year, self.month, self.day)
    }
}
impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hour, self.minute)
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time_string = self.time.map_or(String::from("-----"), |x| format!("{}", x));
        write!(f, "{}  {}", self.date, time_string)
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
