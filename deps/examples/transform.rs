use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Result;
use serde::de;
use serde::Deserialize;
use serde::Deserializer;

#[derive(Debug, Deserialize)]
struct Row {
    color_name: String,
    color: HexColor,
}

#[derive(Debug)]

struct HexColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl FromStr for HexColor {
    type Err = anyhow::Error;

    fn from_str(hex_color: &str) -> std::result::Result<Self, Self::Err> {
        let trimmed = hex_color.trim_matches('#');
        if trimmed.len() != 6 {
            Err(anyhow!("Invalid length of hex string"))
        } else {
            Ok(HexColor {
                red: u8::from_str_radix(&trimmed[..2], 16)?,
                green: u8::from_str_radix(&trimmed[2..4], 16)?,
                blue: u8::from_str_radix(&trimmed[4..6], 16)?,
            })
        }
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

fn main() -> Result<()> {
    let data = "color_name,color
red,#ff0000
green,#00ff00
blue,#0000FF
periwinkle,#ccccff
magenta,#ff00ff"
        .to_owned();
    let mut out = csv::Writer::from_writer(vec![]);
    let mut reader = csv::Reader::from_reader(data.as_bytes());
    // Deserialize as Row, using the implementation above
    for result in reader.deserialize() {
        // We need to provide a type hint for automatic deserialization.
        let res: Row = result?;

        // Serialize the tuple as CSV into Vec<u8>
        out.serialize((
            res.color_name,
            res.color.red,
            res.color.green,
            res.color.blue,
        ))?;
    }
    let written = String::from_utf8(out.into_inner()?)?;
    assert_eq!(Some("magenta,255,0,255"), written.lines().last());
    println!("{}", written);
    Ok(())
}
