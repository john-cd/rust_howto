# Main function

```rust
fn main() {
    println!("Hello, world!");
}
```


## Async Main Function

```rust,ignore
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  Ok(())
}
```

