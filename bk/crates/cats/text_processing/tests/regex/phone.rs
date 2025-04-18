// ANCHOR: example
//! This example demonstrates how to extract and format phone numbers from a
//! text using regular expressions.

use std::fmt;

use anyhow::Result;
use regex::Regex;

/// Represents a US phone number
/// with its area code, exchange code,
/// and subscriber number.
struct PhoneNumber<'a> {
    area: &'a str,
    exchange: &'a str,
    subscriber: &'a str,
}

/// Formats the phone number as "1 (area) exchange-subscriber".
impl fmt::Display for PhoneNumber<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "1 ({}) {}-{}", self.area, self.exchange, self.subscriber)
    }
}

/// Extracts and formats phone numbers from a given text.
///
/// This function defines a regular expression to match phone numbers in
/// various formats. It then iterates over the matches, extracts the area
/// code, exchange code, and subscriber number, and formats them into a
/// `PhoneNumber` struct. Finally, it asserts that the extracted phone
/// numbers match the expected output.
fn main() -> Result<()> {
    let phone_text = "
    +1 505 555 0192 (v) +1 505 555 0112 (c) +1 505 555 0197 (f)
    (202) 555 0134
    Alex 3925550011
    1 (800) 555-1010
    1.299.555.1020";

    // Regular expression to match phone numbers.
    let re = Regex::new(
        r#"(?x)
          (?:\+?1)?                       # Country Code Optional
          [\s\.]?
          (([2-9]\d{2})|\(([2-9]\d{2})\)) # Area Code
          [\s\.\-]?
          ([2-9]\d{2})                    # Exchange Code
          [\s\.\-]?
          (\d{4})                         # Subscriber Number"#,
    )?;

    // Extract phone numbers from the text.
    let phone_numbers = re.captures_iter(phone_text).filter_map(|cap| {
        let groups = (cap.get(2).or(cap.get(3)), cap.get(4), cap.get(5));
        match groups {
            (Some(area), Some(ext), Some(sub)) => Some(PhoneNumber {
                area: area.as_str(),
                exchange: ext.as_str(),
                subscriber: sub.as_str(),
            }),
            _ => None,
        }
    });

    // Assert that the extracted phone numbers match the expected output.
    assert_eq!(
        phone_numbers.map(|m| m.to_string()).collect::<Vec<_>>(),
        vec![
            "1 (505) 555-0192",
            "1 (505) 555-0112",
            "1 (505) 555-0197",
            "1 (202) 555-0134",
            "1 (392) 555-0011",
            "1 (800) 555-1010",
            "1 (299) 555-1020",
        ]
    );

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
