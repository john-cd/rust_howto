// // ANCHOR: example
// // This example demonstrates how to use the `config` crate to
// // configure an application via environment variables.
// //
// // Add to your `Config.toml`:
// // ```toml
// // config = "0.15.6"
// // ```
// use std::collections::HashMap;

// use anyhow::Context;
// use anyhow::Result;

// mod settings {

//     use anyhow::Result;
//     use serde::Deserialize;

//     #[derive(Debug, Default, Deserialize, PartialEq, Eq)]
//     pub struct AppConfig {
//         // A string we'll read from the environment.
//         ip: String,
//         // A list of strings we'll read from the environment.
//         list: Vec<String>,
//     }

//     /// Reads the application's configuration from environment variables.
//     ///
//     /// This function builds a `config::Config` object.
//     pub(crate) fn read_config() -> Result<config::Config> {
//         let config = config::Config::builder()
//             .add_source(
//                 // Add in settings from environment variables (with a prefix
// of                 // `APP`) e.g. `APP_DEBUG=1 ./app` would set
//                 // the `debug` key.
//                 config::Environment::with_prefix("APP")
//                 .try_parsing(true)
//                 // Given a nested configuration such as `redis.password`,
//                 // a separator of _ would allow an environment key of
//                 // `REDIS_PASSWORD` to match.
//                 .separator("_")
//                 // When `list_separator` is set and `try_parsing` is true,
//                 // then all environment variables are parsed
//                 // as `Vec<String>` instead of `String`...
//                 .list_separator(" ")
//                 // ...unless you provide the keys which are `Vec<String>`.
//                 .with_list_parse_key("list"),
//             )
//             .build()?;
//         Ok(config)
//     }
// }

// fn main() -> Result<()> {
//     // Preparation: set environment variables.
//     unsafe {
//         std::env::set_var("APP_LIST", "Item1 Item2");
//         std::env::set_var("APP_IP", "0.0.0.0");
//     }

//     // Read the configuration:
//     let config: config::Config = settings::read_config()?;

//     // Get a specific key:
//     let github: String = config.get("ip").context("Error getting 'ip'")?;
//     println!("ip: {}", github);

//     // Get an array of strings:
//     assert_eq!(
//         config
//             .get_array("list")?
//             .into_iter()
//             .filter_map(|val| val.into_string().ok())
//             .collect::<Vec<String>>(),
//         vec![String::from("Item1"), String::from("Item2")]
//     );

//     // Convert the config object into a HashMap and print it:
//     println!(
//         "\n{:?}\n\n-----------",
//         config.try_deserialize::<HashMap<String, String>>()?
//     );

//     unsafe {
//         std::env::remove_var("APP_LIST");
//         std::env::remove_var("APP_IP");
//     }

//     Ok(())
// }
// // ANCHOR_END: example

// #[test]
// fn test() -> Result<()> {
//     main()?;
//     Ok(())
// }
