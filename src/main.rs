use achievements::{
    config::{self, Day},
    days_since, Interval,
};

fn main() {
    let config = config::read().expect("Failed to read config");

    for Day { label, date: day } in config.days.iter() {
        let days = days_since(*day);
        let achievement = Interval::from_days(days);
        println!("{}: {}", label, achievement);
    }
}
