// ANCHOR: example
use std::collections::HashMap;

use tinytemplate::TinyTemplate;

fn main() {
    // Create a new template engine
    let mut tt = TinyTemplate::new();

    // Define a template string with placeholders
    let template = "Hello, {name}! Welcome to {place}.";

    // Add the template to the engine with a name
    tt.add_template("greeting", template)
        .expect("Failed to add template");

    // Create a context with values for the placeholders
    let mut context = HashMap::new();
    context.insert("name", "Alice");
    context.insert("place", "Rustland");

    // Render the template with the context
    let rendered = tt
        .render("greeting", &context)
        .expect("Failed to render template");

    // Print the rendered template
    println!("{}", rendered);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [P0](https://github.com/john-cd/rust_howto/issues/848)
