// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This example demonstrates how to use the `basic-toml` crate to parse and
// serialize TOML data. //!
// //! Add `basic-toml` to your `Cargo.toml`:
// //! ```
// //! [dependencies]
// //! basic-toml = "0.1.9" # Or latest
// //! ```
// use std::collections::HashMap;
// use std::fs;
// use std::io;

// use basic_toml::from_str;
// use basic_toml::to_string;

// // Define structs that will be serialized/deserialized
// #[derive(Debug, serde::Serialize, serde::Deserialize)]
// struct Config {
//     server: ServerConfig,
//     database: DatabaseConfig,
//     // Vec
//     features: Vec<String>,
//     // Hashmap
//     limits: HashMap<String, i64>,
//     // Custom type
//     #[serde(with = "chrono::serde::ts_seconds")]
//     reset_datetime: chrono::DateTime<chrono::Utc>,
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize)]
// struct ServerConfig {
//     host: String,
//     port: u16,
//     debug: bool,
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize)]
// struct DatabaseConfig {
//     url: String,
//     max_connections: u32,
// }

// /// Helper function to read TOML from file with better error handling.
// fn read_config_file(path: &str) -> Result<Config, io::Error> {
//     let content = fs::read_to_string(path)?;
//     from_str(&content)
//         .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
// }

// /// Helper function to save config to file.
// fn save_config_file(path: &str, config: &Config) -> Result<(), io::Error> {
//     let toml_string = to_string(config)
//         .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
//     fs::write(path, toml_string)
// }

// fn main() -> anyhow::Result<()> {
//     // 1. Parse TOML from a string.
//     let toml_str = r#"
//         [server]
//         host = "127.0.0.1"
//         port = 8080
//         debug = true

//         [database]
//         url = "postgres://user:password@localhost/mydb"
//         max_connections = 50

//         features = ["auth", "api", "webhooks"]

//         [limits]
//         requests_per_minute = 60
//         max_payload_size = 1048576
//     "#;

//     // Parse the TOML string into our `Config` struct.
//     let config: Config = from_str(toml_str)?;
//     println!("Parsed config: {:#?}", config);

//     // 2. Convert back to TOML string.
//     let modified_toml = to_string(&config)?;
//     println!("\nModified TOML:\n{}", modified_toml);

//     // 3. Write TOML to file.
//     fs::write("temp/config.toml", &modified_toml)?;
//     // FIXME use save_config_file

//     // 4. Read TOML from file.
//     let file_content = fs::read_to_string("config.toml")?;
//     let file_config: Config = from_str(&file_content)?;
//     println!("\nConfig from file: {:#?}", file_config);

//     // 5. Error handling.
//     let invalid_toml = r#"
//         [server]
//         host = 127.0.0.1  # Missing quotes around string
//         port = "8080"     # String instead of integer
//     "#;

//     match from_str::<Config>(invalid_toml) {
//         Ok(config) => println!("Successfully parsed: {:?}", config),
//         Err(e) => println!("Parse error: {}", e),
//     }

//     Ok(())
// }

// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// // [finish NOW](https://github.com/john-cd/rust_howto/issues/1095)
