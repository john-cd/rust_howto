// ANCHOR: example
use hyphenation::Language;
use hyphenation::Load;
use hyphenation::Standard;
use std::borrow::Cow;
use textwrap::Options;
use textwrap::WordSplitter;
use textwrap::fill;
use textwrap::wrap;

// Add the dependency to Cargo.toml:
// [dependencies]
// textwrap = "0.16.0"

fn main() {
    let text = "The quick brown fox jumps over the lazy dog. This is an example of text that will be wrapped to demonstrate the textwrap crate functionality in Rust. It offers various options to control how text wrapping behaves.";

    // Basic usage - `wrap` text to a specified width.
    println!("Basic wrap to 40 columns:");
    let wrapped: Vec<Cow<_>> = wrap(text, 40);
    for line in wrapped {
        println!("{}", line);
    }

    // Using `fill` to directly get a string with newlines.
    println!("\nUsing fill with 50 columns:");
    let filled: String = fill(text, 50);
    println!("{}", filled);

    // Customizing `wrap` or `fill` behavior with `Options`.
    println!("\nCustom options (indent of 4 spaces, width of 35):");
    let options = Options::new(35)
        .initial_indent("    ")
        .subsequent_indent("    ");
    let indented = fill(text, &options);
    println!("{}", indented);

    // Controlling hyphenation.
    println!("\nWith hyphenation enabled:");
    let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
    let options =
        Options::new(15).word_splitter(WordSplitter::Hyphenation(dictionary));
    let hyphenated = fill(text, &options);
    println!("{}", hyphenated);

    // Handling long words
    let text_with_long_word =
        "This contains supercalifragilisticexpialidocious which is quite long.";
    println!("\nHandling long words (width: 20):");
    let long_word_wrapped = fill(text_with_long_word, 20);
    println!("{}", long_word_wrapped);

    // Preserving indentation of the first line
    let indented_text =
        "    This paragraph has\nan indentation that should be preserved.";
    println!("\nPreserving first-line indent:");
    let options = Options::new(30).subsequent_indent("    ");
    let preserved = fill(indented_text, &options);
    println!("{}", preserved);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
