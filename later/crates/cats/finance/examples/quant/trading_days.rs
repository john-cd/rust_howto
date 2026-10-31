// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Count NYSE trading days and step forward by a number of trading days with
//! the `trading-calendar` crate.
//!
//! Markets are closed on weekends and holidays, so "ten days from now" in
//! finance means ten trading days. A trading calendar knows these closures. It
//! matters for settlement dates, interest accrual and backtests. NYSE is the
//! New York Stock Exchange.

use anyhow::Result;
use chrono::Days;
use chrono::NaiveDate;
use trading_calendar::Market;
use trading_calendar::TradingCalendar;

/// Counts trading days in the inclusive date range.
///
/// # Errors
///
/// Returns an error if a date lies outside the years the calendar covers.
fn count_trading_days(
    calendar: &TradingCalendar,
    from: NaiveDate,
    through: NaiveDate,
) -> Result<usize> {
    let mut date = from;
    let mut count = 0;
    while date <= through {
        if calendar.is_trading_day(date)? {
            count += 1;
        }
        let Some(next) = date.succ_opt() else {
            break;
        };
        date = next;
    }
    Ok(count)
}

/// Returns the date that is `n` trading days after `from`.
///
/// # Errors
///
/// Returns an error if a date lies outside the years the calendar covers.
fn add_trading_days(
    calendar: &TradingCalendar,
    from: NaiveDate,
    n: usize,
) -> Result<Option<NaiveDate>> {
    let mut date = from;
    let mut remaining = n;
    while remaining > 0 {
        let Some(next) = date.checked_add_days(Days::new(1)) else {
            return Ok(None);
        };
        date = next;
        if calendar.is_trading_day(date)? {
            remaining -= 1;
        }
    }
    Ok(Some(date))
}

/// Counts NYSE trading days across Thanksgiving 2025 and steps past it.
fn main() -> Result<()> {
    let calendar = TradingCalendar::new(Market::NYSE)?;
    let start = NaiveDate::from_ymd_opt(2025, 11, 24).expect("valid date"); // Monday
    let end = NaiveDate::from_ymd_opt(2025, 11, 30).expect("valid date"); // Sunday

    // Monday, Tuesday, Wednesday, and Friday are trading days; weekends and
    // Thanksgiving are skipped. Early closes do not affect the day count.
    let n = count_trading_days(&calendar, start, end)?;
    println!("{n} NYSE trading days between {start} and {end}");
    assert_eq!(n, 4);

    // Three trading days after Wednesday the 26th skips Thanksgiving and the
    // weekend: Fri 28th, Mon Dec 1st, Tue Dec 2nd.
    let wed = NaiveDate::from_ymd_opt(2025, 11, 26).expect("valid date");
    let later = add_trading_days(&calendar, wed, 3)?;
    assert_eq!(later, NaiveDate::from_ymd_opt(2025, 12, 2));
    Ok(())
}

// ANCHOR_END: example

/// Runs the trading days example through its entry point.
///
/// # Errors
///
/// Returns an error if the calendar rejects a date.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the trading days example through its entry point.
    #[test]
    fn test_trading_days_example_entry_point() -> Result<()> {
        main()
    }

    /// Verifies that the calendar knows Independence Day 2025 (a Friday).
    #[test]
    fn independence_day_is_not_a_trading_day() -> Result<()> {
        let calendar = TradingCalendar::new(Market::NYSE)?;
        let holiday = NaiveDate::from_ymd_opt(2025, 7, 4).expect("valid date");
        assert!(!calendar.is_trading_day(holiday)?);
        assert_eq!(
            count_trading_days(
                &calendar,
                NaiveDate::from_ymd_opt(2025, 6, 30).expect("valid date"),
                NaiveDate::from_ymd_opt(2025, 7, 6).expect("valid date"),
            )?,
            4
        );
        Ok(())
    }
}
