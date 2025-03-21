#![allow(dead_code)]
// ANCHOR: example
use inquire::Confirm;
use inquire::Select;
use inquire::Text;

// The `inquire`` crate is used for creating interactive CLI prompts.
fn main() {
    let ans = Confirm::new("Do you want to proceed?")
        .with_default(false)
        .prompt()
        .expect("Failed to read input");
    println!("You answered: {:?}", ans);

    // Prompt for the user's name
    let name = Text::new("What's your name?")
        .prompt()
        .expect("Failed to read input");

    // Prompt for the user's favorite programming language
    let languages = vec!["Rust", "Python", "JavaScript", "C++", "Go", "Other"];
    let favorite_language = Select::new("What's your favorite programming language?", languages)
        //.with_help_message("Hint: it is Rust!")
        .prompt()
        .expect("Failed to read input");

    // Display the collected information
    println!("Hello, {}!", name);
    println!(
        "Your favorite programming language is {}.",
        favorite_language
    );
}
// ANCHOR_END: example

// No tests - this example requires use input
// TODO WIP review NOW how to test
