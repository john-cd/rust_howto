#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Count NYSE trading days and step forward by a number of trading days.

use chrono::NaiveDate;
use trading_calendar::Market;
use trading_calendar::TradingCalendar;

/// Returns the date that is `n` trading days after `from`.
fn add_trading_days(
    calendar: &TradingCalendar,
    from: NaiveDate,
    n: usize,
) -> NaiveDate {
    (0..n).fold(from, |d, _| calendar.next_trading_day(d))
}

/// Counts trading days across the Thanksgiving holiday.
fn main() -> trading_calendar::Result<()> {
    let nyse = TradingCalendar::new(Market::NYSE)?;
    let start = NaiveDate::from_ymd_opt(2025, 11, 24).expect("valid date"); // Monday
    let end = NaiveDate::from_ymd_opt(2025, 11, 30).expect("valid date"); // Sunday

    // Mon, Tue, Wed and the day after Thanksgiving (Friday, early close);
    // Thanksgiving (Thursday 27th) is closed. Weekend days do not count.
    let n = nyse.count_trading_days(start, end)?;
    println!("{n} trading days between {start} and {end}");
    assert_eq!(n, 4);

    // Three trading days after Wednesday the 26th skips Thanksgiving and the
    // weekend: Fri 28th, Mon Dec 1st, Tue Dec 2nd.
    let wed = NaiveDate::from_ymd_opt(2025, 11, 26).expect("valid date");
    let later = add_trading_days(&nyse, wed, 3);
    assert_eq!(later, NaiveDate::from_ymd_opt(2025, 12, 2).expect("valid date"));
    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> trading_calendar::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main().expect("calendar supports 2025");
    }
}
