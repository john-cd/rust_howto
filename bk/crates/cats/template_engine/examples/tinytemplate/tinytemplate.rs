// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example showcasing `tinytemplate`.
//!
//! `TinyTemplate` is a minimal templating library.
//!
//! It was designed with the assumption that the templates are available as
//! static strings, either using string literals or the `include_str!` macro.
//!
//! Although it is possible to use `TinyTemplate` with template
//! strings loaded at runtime, this is not recommended.
//!
//! `TinyTemplate` can only render templates into Strings, not socket or file.

use std::collections::HashMap;

use tinytemplate::TinyTemplate;

/// Renders a template with values from a context and prints the result.
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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the tinytemplate example through its entry point.
    #[test]
    fn test_tinytemplate_example_entry_point() {
        main();
    }

    const TEMPLATE: &str = "Hello, {name}! {{ if flag }}Welcome to {place}.{{ else }}Welcome home!{{ endif }}";

    #[derive(serde::Serialize)]
    struct Context {
        name: &'static str,
        place: &'static str,
        flag: bool,
    }

    fn render(flag: bool) -> String {
        let mut tt = TinyTemplate::new();
        tt.add_template("greeting", TEMPLATE)
            .expect("valid template");
        tt.render(
            "greeting",
            &Context {
                name: "Alice",
                place: "Rustland",
                flag,
            },
        )
        .expect("rendering succeeds")
    }

    /// The true branch is rendered.
    #[test]
    fn renders_the_true_branch() {
        assert_eq!(render(true), "Hello, Alice! Welcome to Rustland.");
    }

    /// The false branch is rendered.
    #[test]
    fn renders_the_false_branch() {
        assert_eq!(render(false), "Hello, Alice! Welcome home!");
    }

    /// A template with unbalanced tags is rejected.
    #[test]
    fn invalid_template_is_rejected() {
        let mut tt = TinyTemplate::new();
        assert!(tt.add_template("bad", "{{ if flag }}never closed").is_err());
    }

    /// Rendering an unknown template is an error.
    #[test]
    fn unknown_template_is_an_error() {
        let tt = TinyTemplate::new();
        assert!(tt.render("nope", &()).is_err());
    }
}
