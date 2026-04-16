#![allow(dead_code)]
// ANCHOR: example
use fancy_regex::Regex;

// `fancy_regex` support features, including backreferences and lookaround, that
// are not available on the `regex` crate.

fn match_two_identical_words(s: &str) {
    // Check if a text consists of two identical words.
    // Note the backreference `\1` to the 1st capture group `(...)`.
    let re = Regex::new(r"^(\w+) (\1)$").expect("Error parsing the regex");
    match re.is_match(s) {
        Ok(true) => println!("Match found."),
        Ok(false) => println!("No match found."),
        Err(err) => eprintln!("Regex error: {err}"),
    }
}

fn capture_groups() {
    let text = "Lorem ipsum dolor sit amet";

    // Capture the word after "dolor" using a look-around. For reference,
    // (?=exp) : look-ahead, succeeds if exp matches to the right of the current
    // position
    // (?!exp) : negative look-ahead, succeeds if exp doesn't match
    // to the right
    // (?<=exp) : look-behind, succeeds if exp matches to the
    // left of the current position
    // (?<!exp) : negative look-behind,
    // succeeds if exp doesn't match to the left.
    let pattern = r"(?<=dolor)\s*(\w+)";

    let re = Regex::new(pattern).expect("Error parsing the regex");
    // Returns the capture groups for the first match in text.
    match re.captures(text) {
        Ok(Some(caps)) => {
            // Get the capture group by its index in the regex.
            if let Some(name) = caps.get(1) {
                println!("Found: {}", name.as_str());
            } else {
                println!("No match found.");
            }
        }
        Ok(None) => println!("No match found."),
        Err(err) => eprintln!("Regex error: {err}"),
    }
}

fn split_text() {
    let re = Regex::new(r"[ \t]+").unwrap();
    let target = "Lorem ipsum\t dolor sit amet";
    let fields: Vec<&str> = re.split(target).map(|x| x.unwrap()).collect();
    assert_eq!(fields, vec!["Lorem", "ipsum", "dolor", "sit", "amet"]);
}

fn main() {
    match_two_identical_words("foo foo");
    capture_groups();
    split_text();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
