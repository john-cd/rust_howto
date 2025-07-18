#![allow(dead_code)]
// ANCHOR: example
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for Rgb {
    type Err = std::num::ParseIntError;

    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        // u8::from_str_radix(src: &str, radix: u32) converts a string
        // slice in a given base to u8
        let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
        let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
        let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;

        Ok(Rgb { r, g, b })
    }
}

fn main() {
    let code: &str = r"#fa7268";
    match Rgb::from_str(code) {
        Ok(rgb) => {
            println!(
                r"The RGB color code is: R: {} G: {} B: {}",
                rgb.r, rgb.g, rgb.b
            );
        }
        Err(_) => {
            println!("{code} is not a valid color hex code!");
        }
    }

    // test whether from_str performs as expected
    assert_eq!(
        Rgb::from_str(r"#fa7268").unwrap(),
        Rgb {
            r: 250,
            g: 114,
            b: 104
        }
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
