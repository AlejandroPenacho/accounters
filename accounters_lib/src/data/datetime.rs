use serde::{Deserialize, Serialize};

use std::str::FromStr;
use time::{
    Date,
    Time,
    Month,
    format_description::well_known::Iso8601
};

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

impl DateTime {
    pub fn simple(date: (i32, u8, u8), time: Option<(u8, u8)>) -> Self {
        DateTime {
            date: Date::from_calendar_date(date.0, Month::January.nth_next(date.1 - 1) , date.2).unwrap(),
            time: time.map(|(h, m)| Time::from_hms(h, m, 0).unwrap()),
        }
    }

    pub fn get_date(&self) -> &Date {
        &self.date
    }

    pub fn get_time(&self) -> &Option<Time> {
        &self.time
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
                date: Date::parse(str, &Iso8601::DEFAULT).map_err(|_| "Whoops")?,
                time: None,
            });
        };
        let date_string = split.next().ok_or("What")?;
        let time_string = split.next().ok_or("What")?;

        Ok(DateTime {
            date: Date::parse(date_string, &Iso8601::DEFAULT).map_err(|_| "Unparsable date")?,
            time: Some(Time::parse(time_string, &Iso8601::DEFAULT).map_err(|_| "Unparsable time")?),
        })
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
            assert!(Time::parse("12:56", &Iso8601::DEFAULT).unwrap() < Time::parse("12:57", &Iso8601::DEFAULT).unwrap());
            assert!(Time::parse("06:52", &Iso8601::DEFAULT).unwrap() < Time::parse("07:04", &Iso8601::DEFAULT).unwrap());
        }

        #[test]
        fn compare_date() {
            assert!(Date::parse("2023-11-08", &Iso8601::DEFAULT).unwrap() < Date::parse("2023-11-09", &Iso8601::DEFAULT).unwrap())
        }

        #[test]
        fn compare_datetime() {
            assert!(DateTime::from_str("2008-08-04") < DateTime::from_str("2008-08-04T00:05"))
        }
    }
}
