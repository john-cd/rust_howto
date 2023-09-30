# Configuration


## Environment variables

[dotenv]( https://crates.io/crates/dotenv )


## Config

[config]( https://crates.io/crates/config ): layered configuration system for Rust applications. Read from JSON, TOML, YAML, INI, RON, JSON5 files.



## Confy

[Confy]( https://docs.rs/confy/latest/confy/index.html )

```rust
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MyConfig {
    version: u8,
    api_key: String,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for MyConfig {
    fn default() -> Self { Self { version: 0, api_key: "".into() } }
}

fn main() -> Result<(), confy::ConfyError> {
    let cfg = confy::load("my-app-name", None)?;
    // confy::store("my-app-name", None, cfg)?;
    Ok(())
}
```
