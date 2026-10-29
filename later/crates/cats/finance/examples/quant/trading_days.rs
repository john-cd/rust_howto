// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Count sample trading days and step forward by a number of trading days.

use std::collections::HashSet;

use chrono::Datelike;
use chrono::Days;
use chrono::NaiveDate;
use chrono::Weekday;

/// Returns whether `date` is a weekday that is not in `holidays`.
fn is_trading_day(date: NaiveDate, holidays: &HashSet<NaiveDate>) -> bool {
    !matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
        && !holidays.contains(&date)
}

/// Counts trading days in the inclusive date range.
fn count_trading_days(
    from: NaiveDate,
    through: NaiveDate,
    holidays: &HashSet<NaiveDate>,
) -> usize {
    let mut date = from;
    let mut count = 0;
    while date <= through {
        if is_trading_day(date, holidays) {
            count += 1;
        }
        let Some(next) = date.succ_opt() else {
            break;
        };
        date = next;
    }
    count
}

/// Returns the date that is `n` trading days after `from`.
fn add_trading_days(
    from: NaiveDate,
    n: usize,
    holidays: &HashSet<NaiveDate>,
) -> Option<NaiveDate> {
    let mut date = from;
    let mut remaining = n;
    while remaining > 0 {
        date = date.checked_add_days(Days::new(1))?;
        if is_trading_day(date, holidays) {
            remaining -= 1;
        }
    }
    Some(date)
}

/// Counts weekdays while skipping the supplied Thanksgiving holiday.
fn main() {
    let thanksgiving =
        NaiveDate::from_ymd_opt(2025, 11, 27).expect("valid date");
    let holidays = HashSet::from([thanksgiving]);
    let start = NaiveDate::from_ymd_opt(2025, 11, 24).expect("valid date"); // Monday
    let end = NaiveDate::from_ymd_opt(2025, 11, 30).expect("valid date"); // Sunday

    // Monday, Tuesday, Wednesday, and Friday are trading days; weekends and
    // Thanksgiving are skipped. Early closes do not affect the day count.
    let n = count_trading_days(start, end, &holidays);
    println!("{n} trading days between {start} and {end}");
    assert_eq!(n, 4);

    // Three trading days after Wednesday the 26th skips Thanksgiving and the
    // weekend: Fri 28th, Mon Dec 1st, Tue Dec 2nd.
    let wed = NaiveDate::from_ymd_opt(2025, 11, 26).expect("valid date");
    let later = add_trading_days(wed, 3, &holidays).expect("date range fits");
    assert_eq!(
        later,
        NaiveDate::from_ymd_opt(2025, 12, 2).expect("valid date")
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the trading days example through its entry point.
    #[test]
    fn test_trading_days_example_entry_point() {
        main();
    }
}
