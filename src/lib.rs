use std::fmt::Display;

use time::OffsetDateTime;

pub mod config;

pub const YEAR: usize = 365;
pub const MONTH: usize = 30;
pub const WEEK: usize = 7;

pub const DAY_IN_SECONDS: usize = 24 * 60 * 60;

#[derive(PartialEq, Debug)]
pub enum IntervalEnum {
    Day(usize),
    Week(usize),
    Month(usize),
    Year(usize),
    Decade(usize),
}

#[derive(PartialEq, Debug)]
pub struct Interval {
    days: usize,
    e: IntervalEnum,
}

impl Interval {
    pub fn from_days(days: usize) -> Self {
        Self {
            days,
            e: Self::enum_from_days(days),
        }
    }

    fn enum_from_days(days: usize) -> IntervalEnum {
        if days == 0 {
            return IntervalEnum::Day(0);
        }

        if days % YEAR == 0 {
            let years = days / YEAR;
            if years % 10 == 0 {
                let decades = years / 10;
                return IntervalEnum::Decade(decades);
            };
            return IntervalEnum::Year(years);
        }

        if days % MONTH == 0 {
            return IntervalEnum::Month(days / MONTH);
        };

        if days % WEEK == 0 {
            return IntervalEnum::Week(days / WEEK);
        };

        IntervalEnum::Day(days)
    }

    pub fn to_words(&self) -> String {
        match self.e {
            IntervalEnum::Decade(1) => "1 decade, that's amazing".to_string(),
            IntervalEnum::Decade(d) => format!("{d} decades"),
            IntervalEnum::Year(1) => "1 year, happy anniversary!".to_string(),
            IntervalEnum::Year(y) => format!("{y} years"),
            IntervalEnum::Month(1) => "1 month".to_string(),
            IntervalEnum::Month(m) => format!("{m} months"),
            IntervalEnum::Week(1) => "1 week".to_string(),
            IntervalEnum::Week(w) => format!("{w} weeks"),
            IntervalEnum::Day(0) => "Recently".to_string(),
            IntervalEnum::Day(1) => "1 day".to_string(),
            IntervalEnum::Day(d) => format!("{d} days"),
        }
    }

    fn badges(&self) -> String {
        match self.days {
            d if d >= 10 * YEAR => {
                let decades = d / (10 * YEAR);
                String::from("💎").repeat(decades)
            }
            d if d >= YEAR => {
                let years = d / YEAR;
                String::from("🌟").repeat(years)
            }
            d if d >= MONTH => {
                let months = d / MONTH;
                String::from("⭐").repeat(months)
            }
            d if d >= WEEK => {
                let weeks = d / WEEK;
                String::from("★").repeat(weeks)
            }
            d if d >= 1 => String::from("☆").repeat(d),
            _ => String::new(),
        }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.to_words(), self.badges())
    }
}

pub fn days_since(day: OffsetDateTime) -> usize {
    let now = OffsetDateTime::now_utc();
    let seconds_elapsed: time::Duration = now - day;

    seconds_elapsed.as_seconds_f64() as usize / DAY_IN_SECONDS
}

#[test]
fn test_from_days() {
    const DECADE: usize = 10 * YEAR;
    assert_eq!(IntervalEnum::Decade(3), Interval::from_days(3 * DECADE).e);
    assert_eq!(IntervalEnum::Decade(1), Interval::from_days(1 * DECADE).e);

    assert_eq!(IntervalEnum::Year(33), Interval::from_days(33 * YEAR).e);
    assert_eq!(IntervalEnum::Year(11), Interval::from_days(11 * YEAR).e);
    assert_eq!(IntervalEnum::Year(5), Interval::from_days(5 * YEAR).e);
    assert_eq!(IntervalEnum::Year(1), Interval::from_days(1 * YEAR).e);

    assert_eq!(IntervalEnum::Month(5), Interval::from_days(5 * MONTH).e);
    assert_eq!(IntervalEnum::Month(1), Interval::from_days(MONTH).e);

    assert_eq!(IntervalEnum::Week(3), Interval::from_days(3 * WEEK).e);
    assert_eq!(IntervalEnum::Week(1), Interval::from_days(WEEK).e);

    assert_eq!(IntervalEnum::Day(15), Interval::from_days(15).e);
    assert_eq!(IntervalEnum::Day(10), Interval::from_days(10).e);
    assert_eq!(IntervalEnum::Day(5), Interval::from_days(5).e);
    assert_eq!(IntervalEnum::Day(1), Interval::from_days(1).e);
    assert_eq!(IntervalEnum::Day(0), Interval::from_days(0).e);
}

#[test]
fn test_to_words() {
    assert_eq!("3 decades", Interval::from_days(3 * 10 * YEAR).to_words());
    assert_eq!(
        "1 decade, that's amazing",
        Interval::from_days(10 * YEAR).to_words()
    );

    assert_eq!("33 years", Interval::from_days(33 * YEAR).to_words());
    assert_eq!("11 years", Interval::from_days(11 * YEAR).to_words());
    assert_eq!("5 years", Interval::from_days(5 * YEAR).to_words());
    assert_eq!(
        "1 year, happy anniversary!",
        Interval::from_days(YEAR).to_words()
    );

    assert_eq!("5 months", Interval::from_days(5 * MONTH).to_words());
    assert_eq!("1 month", Interval::from_days(1 * MONTH).to_words());

    assert_eq!("3 weeks", Interval::from_days(3 * WEEK).to_words());
    assert_eq!("1 week", Interval::from_days(1 * WEEK).to_words());

    assert_eq!("15 days", Interval::from_days(15).to_words());
    assert_eq!("10 days", Interval::from_days(10).to_words());
    assert_eq!("5 days", Interval::from_days(5).to_words());
    assert_eq!("1 day", Interval::from_days(1).to_words());
    assert_eq!("Recently", Interval::from_days(0).to_words());
}

#[test]
fn test_to_string() {
    assert_eq!(
        "3 decades 💎💎💎",
        Interval::from_days(3 * 10 * YEAR).to_string()
    );
    assert_eq!(
        "1 decade, that's amazing 💎",
        Interval::from_days(10 * YEAR).to_string()
    );

    assert_eq!(
        "33 years 💎💎💎",
        Interval::from_days(33 * YEAR).to_string()
    );
    assert_eq!("11 years 💎", Interval::from_days(11 * YEAR).to_string());
    assert_eq!(
        "5 years 🌟🌟🌟🌟🌟",
        Interval::from_days(5 * YEAR).to_string()
    );
    assert_eq!(
        "1 year, happy anniversary! 🌟",
        Interval::from_days(YEAR).to_string()
    );

    assert_eq!(
        "5 months ⭐⭐⭐⭐⭐",
        Interval::from_days(5 * MONTH).to_string()
    );
    assert_eq!("1 month ⭐", Interval::from_days(1 * MONTH).to_string());

    assert_eq!("3 weeks ★★★", Interval::from_days(3 * WEEK).to_string());
    assert_eq!("1 week ★", Interval::from_days(WEEK).to_string());

    assert_eq!("15 days ★★", Interval::from_days(15).to_string());
    assert_eq!("10 days ★", Interval::from_days(10).to_string());
    assert_eq!("5 days ☆☆☆☆☆", Interval::from_days(5).to_string());
    assert_eq!("1 day ☆", Interval::from_days(1).to_string());
    assert_eq!("Recently ", Interval::from_days(0).to_string());
}
