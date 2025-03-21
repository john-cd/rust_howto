// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// use std::fmt;

// use anyhow::anyhow;
// use anyhow::Context;
use anyhow::Result;
// use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// struct Rgb {
//     red: u8,
//     blue: u8,
//     green: u8,
// }

// impl Rgb {
//     fn from_reader(csv_data: &[u8]) -> Result<Rgb> {
//         let c = csv::Reader::from_reader(csv_data)
//             .deserialize()
//             .nth(0)
//             .ok_or(anyhow!(""))?;
//         let color = c.context("")?;

//         Ok(color)
//     }
// }

// impl fmt::UpperHex for Rgb {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let hexa = u32::from(self.red) << 16
//             | u32::from(self.blue) << 8
//             | u32::from(self.green);
//         write!(f, "{:X}", hexa)
//     }
// }

// fn main() -> Result<()> {
//     let csv = "red,blue,green
// 102,256,204";

//     let rgb = Rgb::from_reader(csv.as_bytes())?;
//     println!("{:?} to hexadecimal #{:X}", rgb, rgb);

//     Ok(())
// }

fn main() -> Result<()> {
    Ok(())
}

#[test]
fn test() {
    let res = main();
    println!("{:?}", res);
    assert!(res.is_ok());
}

// [review rewrite NOW](https://github.com/john-cd/rust_howto/issues/168)
