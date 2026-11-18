// TODO Reviewed ??-??-2026.

// ANCHOR: example
/// Couples the result to both input lifetimes.
fn choose<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}

/// Runs the example demonstrated by this module.
fn main() {
    let chosen;
    {
        let temporary = String::from("temporary");
        chosen = choose("static input", &temporary);
    }
    println!("{chosen}");
}
// ANCHOR_END: example
