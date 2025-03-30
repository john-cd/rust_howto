// ANCHOR: example
use std::fmt;

use anyhow::Result;
use anyhow::anyhow;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Rgb {
    red: u8,
    blue: u8,
    green: u8,
}

impl Rgb {
    fn from_csv(csv_data: &[u8]) -> Result<Rgb> {
        let color = csv::Reader::from_reader(csv_data)
            .deserialize()
            .nth(0)
            .ok_or(anyhow!("First CSV row does not exist."))?; // Convert `None` into `Result::Err`.
        Ok(color?)
    }
}

impl fmt::UpperHex for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hexa = u32::from(self.red) << 16
            | u32::from(self.blue) << 8
            | u32::from(self.green);
        write!(f, "{:X}", hexa)
    }
}

fn main() -> Result<()> {
    let csv = "red,blue,green
102,256,204"; // Note the invalid value.

    let rgb = Rgb::from_csv(csv.as_bytes())?;
    println!("{:?} to hexadecimal #{:X}", rgb, rgb);

    Ok(())
}
// ANCHOR_END: example

#[should_panic]
#[test]
fn test() {
    main();
}
