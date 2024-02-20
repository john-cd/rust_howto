use ansi_term::Style;

#[test]
fn test() {
    println!(
        "{} and this is not",
        Style::new().bold().paint("This is Bold")
    );
}
