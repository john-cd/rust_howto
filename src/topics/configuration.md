# Configuration


## Environment variables

[dotenvy]( https://crates.io/crates/dotenvy )

It supersedes [dotenv]( https://crates.io/crates/dotenv )

```rust,ignore
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file.
    // Fails if .env file not found, not readable or invalid.
    dotenvy::dotenv()?;

    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }

    Ok(())
}
```

To retrieve a single environment variable

```rust,ignore
use std::env;

#fn main() {
#    println!(env_extract());
#}
#
#fn env_extract() -> String {

let log_env_var = env::var("RUST_LOG").unwrap_or_else(|_| {"debug".into() });

let user_env_var = env::var("USER").expect("$USER is not set");

let shell = env!("SHELL", "$SHELL is not set"); // inspect an environment variable at compile-time.

let optional_value = option_env!("SHELL"); 

return optional_value
        .unwrap_or("no shell set")
        .to_string();
#}
```

[https://www.thorsten-hans.com/working-with-environment-variables-in-rust/]( https://www.thorsten-hans.com/working-with-environment-variables-in-rust/ )


### Envy

Envy can deserialize environment variables into typesafe struct.

```toml
[dependencies]
envy = "0.4"
serde = { version = "1.0", features = ["derive"] }
```

```rust,ignore
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Configuration {
    port: u16,
    items_per_page: u16
}

let c = envy::from_env::<Configuration>()
        .expect("Please provide PORT and ITEMS_PER_PAGE env vars");

let c = envy::prefixed("MY_APP__")
        .from_env::<Configuration>()
        .expect("Please provide MY_APP__PORT and MY_APP__ITEMS_PER_PAGE env vars");
```


## Config

[config]( https://crates.io/crates/config ): layered configuration system for Rust applications. Read from JSON, TOML, YAML, INI, RON, JSON5 files.


## Confy

[Confy]( https://docs.rs/confy/latest/confy/index.html )

```rust,ignore
use serde::{Serialize, Deserialize};

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
