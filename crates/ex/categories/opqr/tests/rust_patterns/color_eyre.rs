// ANCHOR: example
use color_eyre::eyre::Result;

// `color_eyre` provides beautiful error reporting and backtraces with colors.
// It's especially useful for debugging.

fn main() -> Result<()> {
    color_eyre::install()?;

    let result = divide(10, 0)?;
    println!("Result: {}", result);

    Ok(())
}

fn divide(a: i32, b: i32) -> Result<i32> {
    if b == 0 {
        Err(color_eyre::eyre::eyre!("Division by zero"))
    } else {
        Ok(a / b)
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    assert!(main().is_err());
}
