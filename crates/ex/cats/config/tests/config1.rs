// // ANCHOR: example
// // ANCHOR_END: example

// use std::collections::HashMap;
// use std::fs;

// use anyhow::Context;
// use anyhow::Result;
// use config::Config;
// // FIXME use config::Environment;
// use serde_derive::Deserialize;

// // `Config` is prioritized configuration repository.
// // It maintains a set of configuration sources,
// // fetches values to populate those,
// // and provides them according to the source’s priority.

// fn read_config() -> Result<Config> {
//     let config = Config::builder()
//         // Add in `config.toml`
//         .add_source(config::File::with_name("temp/config.toml"))
//         // Also read environment variables
//         .add_source(
//             // An environment source collects a dictionary of environment
// variables values             // into a hierarchical config Value type.
//             //  Here, we read only env vars that begin with the defined
// prefix.             config::Environment::with_prefix("APP")
//                 .try_parsing(true)
//                 // Given a nested configuration such as `redis.password`,
//                 // a separator of _ would allow an environment key of
// REDIS_PASSWORD to match.                 .separator("_")
//                 // When set and `try_parsing` is true,
//                 // then all environment variables will be parsed
//                 // as Vec<String> instead of String.
//                 .list_separator(" "),
//         )
//         .build()?;

//     Ok(config)
// }

// #[derive(Debug, Default, Deserialize, PartialEq, Eq)]
// struct AppConfig {
//     // A string we'll read from a TOML file and the environment.
//     ip: String,
//     // A map of strings we'll read from a TOML file.
//     keys: HashMap<String, String>,
//     // A list of strings we'll read from the environment.
//     list: Vec<String>,
// }

// fn main() -> Result<()> {
//     // Prep: set an environment variable
//     unsafe {
//         std::env::set_var("APP_LIST", "Item1 Item2");
//     }

//     // Prep: create a TOML config file
//     let toml = br#"
//    ip = '127.0.0.1'

//    [keys]
//    github = 'xxxxxxxxxxxxxxxxx'
//    travis = 'yyyyyyyyyyyyyyyyy'
// "#;
//     if !fs::exists("temp")? {
//         fs::create_dir("temp")?;
//     }
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
//         "\n{:?} \n\n-----------",
//         config.try_deserialize::<HashMap<String, String>>()?
//     );

//     // Attempt to deserialize the entire configuration into a custom type.
//     let app: AppConfig = config2.try_deserialize()?;

//     println!("App config: {:?}", app);

//     unsafe {
//         std::env::remove_var("APP_LIST");
//     }

//     Ok(())
// }

// [ P0 finish](https://github.com/john-cd/rust_howto/issues/1012)
// #[test]
// fn test_config() -> Result<(), config::ConfigError> {
//     // Alternate source for the environment.
//     // This can be used when you want to test your own code using this
// source,     // without the need to change the actual system environment
// variables.     // let source = Environment::default().source(Some({
//     //     let mut env = HashMap::new();
//     //     env.insert("APP_LIST".into(), "my-value".into());
//     //     env
//     // }));

//     //   let config: MyConfig = Config::builder()
//     //     .add_source(source)
//     //     .build()?
//     //     .try_into()?;
//     //   assert_eq!(config.my_string, "my-value");

//     Ok(())
// }

// #[test]
// fn test() -> Result<()> {
//     main()
// }
