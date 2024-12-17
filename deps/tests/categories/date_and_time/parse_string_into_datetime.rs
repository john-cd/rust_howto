// ANCHOR: example
use chrono::DateTime;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use chrono::format::ParseError;

fn main() -> Result<(), ParseError> {
    let rfc2822 =
        DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;
    println!("{}", rfc2822);

    let rfc3339 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
    println!("{}", rfc3339);

    let custom = DateTime::parse_from_str(
        "5.8.1994 8:00 am +0000",
        "%d.%m.%Y %H:%M %P %z",
    )?;
    println!("{}", custom);

    let time_only = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S")?;
    println!("{}", time_only);

    let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
    println!("{}", date_only);

    let no_timezone = NaiveDateTime::parse_from_str(
        "2015-09-05 23:56:04",
        "%Y-%m-%d %H:%M:%S",
    )?;
    println!("{}", no_timezone);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
