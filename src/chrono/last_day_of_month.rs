use ::chrono::{DateTime, Utc, Datelike};

pub trait LastDayOfMonth {
    fn is_leap_year(&self) -> bool;

    fn last_day_of_month(&self) -> u32;
}

impl LastDayOfMonth for DateTime<Utc> {
    fn is_leap_year(&self) -> bool {
        self.naive_utc().year() % 4 == 0 && (self.naive_utc().year() % 100 != 0 || self.naive_utc().year() % 400 == 0)
    }

    fn last_day_of_month(&self) -> u32 {
        match self.naive_utc().month() {
            4 | 6 | 9 | 11 => 30,
            2 => if self.is_leap_year() {
                29
            } else {
                28
            },
            _ => 31,
        }
    }
}
