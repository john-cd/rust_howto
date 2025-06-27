#![allow(dead_code)]
// ANCHOR: example
//! Example showcasing `tinytemplate`.
//!
//! TinyTemplate is a minimal templating library.
//!
//! It was designed with the assumption that the templates are available as
//! static strings, either using string literals or the `include_str!` macro.
//!
//! Although it is possible to use TinyTemplate with template
//! strings loaded at runtime, this is not recommended.
//!
//! TinyTemplate can only render templates into Strings, not socket or file.

use std::collections::HashMap;

use tinytemplate::TinyTemplate;

fn main() {
    // Create a new template engine:
    let mut tt = TinyTemplate::new();

    // Define a template string with {placeholders}:
    let template = "Hello, {name}! {{ if flag }}Welcome to {place}.{{ else }}Welcome home!{{ endif }}";
    // Syntax summary:
    // Rendering values - { myvalue }
    // Conditionals - {{ if foo }}Foo is true{{ else }}Foo is false{{ endif }}
    // Loops - {{ for value in row }}{value}{{ endfor }}
    // Customizable value formatters { value | my_formatter }
    // Macros {{ call my_template with foo }}

    // Add the template to the engine with a name:
    tt.add_template("greeting", template)
        .expect("Failed to add template");

    // Create a context with values for the placeholders:
    let mut context = HashMap::new();
    context.insert("name", "Alice");
    context.insert("place", "Rustland");
    context.insert("flag", "true");

    // Render the template with the context:
    let rendered = tt
        .render("greeting", &context)
        .expect("Failed to render template");

    // Print the rendered template:
    println!("{rendered}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
