use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Hash)]
pub struct DateTime {
    date: Date,
    time: Option<Time>,
}

#[derive(Deserialize, Serialize, Hash)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Deserialize, Serialize, Hash)]
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
