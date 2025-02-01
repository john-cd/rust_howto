// // HACK: the `config` package has the same name
// // than the current crate, thus we renamed it `config1`
// // See `Cargo.toml`
// use config1 as config;
// // ANCHOR: example
// use std::collections::HashMap;
// use std::fs;

// use anyhow::Context;
// use serde::Deserialize;

// #[derive(Debug, Default, Deserialize, PartialEq, Eq)]
// struct AppConfig {
//     // A string we'll read from a TOML file and the environment.
//     ip: String,
//     // A map of strings we'll read from a TOML file.
//     keys: HashMap<String, String>,
//     // A list of strings we'll read from the environment.
//     list: Vec<String>,
// }

// fn read_config() -> anyhow::Result<config::Config> {
//     let config = config::Config::builder()
//         // Add in (optional) `config.toml`
//         // File::with_name(..) is shorthand for File::from(Path::new(..))
//         // This could be a JSON, YAML, INI file,
//         // or even a file with a custom format
//         .add_source(config::File::with_name("temp/config.toml")
//              .required(false))
//          .add_source(
//          // Add in settings from environment variables (with a prefix of APP)
//          // Eg.. `APP_DEBUG=1 ./app` would set the `debug` key
//             config::Environment::with_prefix("APP")
//                 // Try parsing the env. variables as bool, i64, and f64
//                 .try_parsing(true)
//                 // Given a nested configuration such as `redis.password`,
//                 // a separator of _ would allow an environment key of
//                 // REDIS_PASSWORD to match.
//                 .separator("_")
//                 // When set and `try_parsing` is true,
//                 // then all environment variables are parsed
//                 // as Vec<String> instead of String...
//                 .list_separator(" ")
//                 // ...unless you provide the keys which should be Vec<String>
//                 .with_list_parse_key("list")
//         )
//         // You may also programmatically change settings
//         //.set_override("database.url", "postgres://")?
//         .build()?;
//     Ok(config)
// }

// fn main() -> anyhow::Result<()> {
//     // Prep: set environment variables
//     unsafe {
//         std::env::set_var("APP_LIST", "Item1 Item2");
//         std::env::set_var("APP_IP", "0.0.0.0");
//     }

//     // Prep: create a TOML config file
//     let toml = br#"
//    ip = '127.0.0.1'

//    [keys]
//    github = 'xxxxxxxxxxxxxxxxx'
//    travis = 'yyyyyyyyyyyyyyyyy'
// "#;
//     // Create the temp folder, if needed
//     if !fs::exists("temp")? {
//         fs::create_dir("temp")?;
//     }
//     // ...and create a TOML file
//     std::fs::write("temp/config.toml", toml)?;

//     // Read the configuration
//     let config = read_config()?;

//     // Get a specific key
//     let github: String = config
//         .get("keys.github")
//         .context("Error getting 'keys.github'")?;
//     println!("keys.github: {}", github);

//     // Get an array of strings
//     assert_eq!(
//         config
//             .get_array("list")?
//             .into_iter()
//             .filter_map(|val| val.into_string().ok())
//             .collect::<Vec<String>>(),
//         vec![String::from("Item1"), String::from("Item2")]
//     );

//     let config2 = config.clone();

//     // Print the configuration
//     println!(
//         "\n{:?}\n\n-----------",
//         config.try_deserialize::<HashMap<String, String>>()?
//     );

//     // Attempt to deserialize the entire configuration into a custom type.
//     let app: AppConfig = config2.try_deserialize()?;

//     println!("App config: {:?}", app);

//     unsafe {
//         std::env::remove_var("APP_LIST");
//         std::env::remove_var("APP_IP");
//     }

//     Ok(())
// }
// // ANCHOR_END: example

// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// // https://github.com/rust-cli/config-rs/tree/main/examples
