#![allow(dead_code)]
// ANCHOR: example
use inquire::Confirm;
use inquire::Select;
use inquire::Text;

/// The `inquire` crate is used for creating interactive CLI prompts.
fn main() {
    // Prompt the user with a confirmation question.
    let ans = Confirm::new("Do you want to proceed?")
        .with_default(false)
        .prompt()
        .expect("Failed to read input");
    println!("You answered: {:?}", ans);

    // Prompt the user for their name.
    let name = Text::new("What's your name?")
        .prompt()
        .expect("Failed to read input");

    // Define a list of programming languages.
    let languages = vec!["Rust", "Python", "JavaScript", "C++", "Go", "Other"];
    // Prompt the user to select their favorite programming language from the
    // list.
    let favorite_language =
        Select::new("What's your favorite programming language?", languages)
            .prompt()
            .expect("Failed to read input");

    // Display the collected information to the user.
    println!("Hello, {}!", name);
    println!(
        "Your favorite programming language is {}.",
        favorite_language
    );
}
// ANCHOR_END: example

#[test]
#[ignore = "Requires user input"]
fn require_user_input() {
    main();
}
