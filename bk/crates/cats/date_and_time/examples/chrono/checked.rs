#![allow(dead_code)]
// ANCHOR: example
use chrono::DateTime;
use chrono::Duration;
use chrono::TimeDelta;
use chrono::Utc;

/// Returns the date and time one day earlier than the given date and time, or
/// None if the operation would overflow.
fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(Duration::try_days(1).unwrap())
}

/// Demonstrates checked arithmetic with `DateTime` and `Duration`.
fn main() {
    let now = Utc::now();
    println!("{now}");

    let almost_three_weeks_from_now = now
        .checked_add_signed(Duration::try_weeks(2).unwrap())
        .and_then(|in_2weeks| {
            in_2weeks.checked_add_signed(Duration::try_weeks(1).unwrap())
        })
        .and_then(day_earlier);

    match almost_three_weeks_from_now {
        Some(x) => println!("{x}"),
        None => eprintln!("Almost three weeks from now overflows!"),
    }

    match now.checked_add_signed(TimeDelta::MAX) {
        Some(x) => println!("{x}"),
        None => eprintln!(
            "We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."
        ),
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
