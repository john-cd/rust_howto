// TODO
// use std::fmt;

// use anyhow::Result;
// use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// struct Rgb {
//     red: u8,
//     blue: u8,
//     green: u8,
// }

// impl Rgb {
//     fn from_reader(csv_data: &[u8]) -> Result<Rgb> {
//         let color: Rgb = csv::Reader::from_reader(csv_data)
//             .deserialize()
//             .nth(0)
//             .ok_or("Cannot deserialize the first CSV record")?;
//         //.chain_err(|| "Cannot deserialize RGB color")?;

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

// fn run() -> Result<()> {
//     let csv = "red,blue,green
// 102,256,204";

//     let rgb = Rgb::from_reader(csv.as_bytes()).chain_err(|| {
//         "Cannot
// read CSV data"
//     })?;
//     println!("{:?} to hexadecimal #{:X}", rgb, rgb);

//     Ok(())
// }

// #[test]
// fn test() {
//     if let Err(ref errors) = run() {
//         // TODO
//         // eprintln!("Error level - description");
//         // errors
//         //     .iter()
//         //     .enumerate()
//         //     .for_each(|(index, error)| eprintln!("└> {} - {}",
// index,         // error));

//         if let Some(backtrace) = errors.backtrace() {
//             eprintln!("{:?}", backtrace);
//         }

//         // In a real use case, errors should handled. For
//         // example:
//         // ::std::process::exit(1);
//     }
// }
