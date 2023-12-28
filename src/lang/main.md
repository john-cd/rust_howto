# Main function

```rust,editable
fn main() {
    println!("Hello, world!");
}
```

## Async Main Function

```rust,editable,ignore,mdbook-runnable
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("I'm async!");
  Ok(())
}
```
