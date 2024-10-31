use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;

fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(Duration::try_days(1).unwrap())
}

fn main() {
    let now = Utc::now();
    println!("{}", now);

    let almost_three_weeks_from_now = now
        .checked_add_signed(Duration::try_weeks(2).unwrap())
        .and_then(|in_2weeks| {
            in_2weeks.checked_add_signed(Duration::try_weeks(1).unwrap())
        })
        .and_then(day_earlier);

    match almost_three_weeks_from_now {
        Some(x) => println!("{}", x),
        None => eprintln!("Almost three weeks from now overflows!"),
    }

    match now.checked_add_signed(Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!(
            "We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."
        ),
    }
}

#[test]
fn test() {
    main();
}
