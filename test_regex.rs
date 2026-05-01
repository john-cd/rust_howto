use regex::Regex;
use once_cell::sync::Lazy;

static EXTRACT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)```rust.*?\n(?<code>.*?)```").unwrap());

fn main() {
    let buf = "\
```rust
fn main() {
    println!(\"hello\");
}
```
";

    for (number, (_, [code])) in EXTRACT_REGEX
        .captures_iter(&buf)
        .map(|c| c.extract())
        .enumerate()
    {
        println!("Code {}: {}", number, code);
    }
}
