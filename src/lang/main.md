# Main function

```rust
fn main() {
    println!("Hello, world!");
}
```

## Async Main Function

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  Ok(())
}
```

