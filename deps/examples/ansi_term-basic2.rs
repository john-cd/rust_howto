use ansi_term::Colour;
use ansi_term::Style;

fn main() {
    println!(
        "{}, {} and {}",
        Colour::Yellow.paint("This is colored"),
        Style::new().bold().paint("this is bold"),
        Colour::Yellow.bold().paint("this is bold and colored")
    );
}
