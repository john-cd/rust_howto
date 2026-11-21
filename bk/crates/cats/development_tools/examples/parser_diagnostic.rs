// TODO Reviewed ??-??-2026.
//! Renders source spans and a hint for a small parser diagnostic.

// ANCHOR: example
use annotate_snippets::AnnotationKind;
use annotate_snippets::Level;
use annotate_snippets::Renderer;
use annotate_snippets::Snippet;

/// Shows the offending token, its related declaration, and a repair hint.
fn main() {
    let source = "port = many\n";
    let report = [Level::ERROR
        .primary_title("expected a port number")
        .element(
            Snippet::source(source)
                .path("settings.conf")
                .annotation(AnnotationKind::Primary.span(7..11).label("expected digits"))
                .annotation(AnnotationKind::Context.span(0..4).label("port setting")),
        )
        .element(Level::HELP.message("use an integer such as 8080"))];
    let diagnostic = Renderer::plain().render(&report);
    assert!(diagnostic.contains("expected digits"));
    assert!(diagnostic.contains("port setting"));
    assert!(diagnostic.contains("use an integer such as 8080"));
    println!("{diagnostic}");
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Checks the rendered diagnostic includes both labels and the hint.
    #[test]
    fn renders_labels_and_hint() {
        super::main();
    }
}
