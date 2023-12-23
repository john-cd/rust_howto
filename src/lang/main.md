# Main function

```rust,editable
fn main() {
    println!("Hello, world!");
}
```

## Async Main Function

```rust,editable,ignore
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  Ok(())
}
```
