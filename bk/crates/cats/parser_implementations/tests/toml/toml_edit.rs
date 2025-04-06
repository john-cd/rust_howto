// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This example demonstrates how to use the `toml_edit` crate to
// //! parse, modify, and print TOML documents while preserving the
// //! original formatting.
// //!
// //! Use `toml_edit` instead of `toml` for format-preserving editing or finer
// control over output. use toml_edit::DocumentMut;
// use toml_edit::Item;
// use toml_edit::value;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Parse existing TOML.
//     let toml_content = r#"
// [package]
// name = "my-package"
// version = "0.1.0"
// authors = ["Me <me@example.com>"]

// [dependencies]
// serde = "1.0"
// tokio = { version = "1.0", features = ["full"] }

// [[bin]]
// name = "app"
// path = "src/main.rs"
// "#;

//     // Parse the document.
//     let mut doc = toml_content.parse::<DocumentMut>()?;

//     // Modify values.
//     doc["package"]["version"] = value("0.2.0");

//     // Add a new dependency.
//     doc["dependencies"]["toml_edit"] = value("0.22.24");

//     // Add a feature to an existing dependency.
//     if let Some(Item::Value(v)) = doc
//         .get_mut("dependencies")
//         .and_then(|deps| deps.get_mut("tokio"))
//     {
//         if let Some(t) = v.as_inline_table_mut() {
//             let features = t
//                 .get_mut("features")
//                 .and_then(|f| f.as_array_mut())
//                 .expect("`tokio` features should be an array");

//             features.push("time");
//         }
//     }

//     // Add a new section
//     let mut dev_deps = toml_edit::table();
//     dev_deps.insert("pretty_assertions", value("0.7.2"));
//     doc.insert("dev-dependencies", Item::Table(dev_deps));

//     // Add to array of tables
//     if let Some(Item::ArrayOfTables(bins)) = doc.get_mut("bin") {
//         let mut new_bin = toml_edit::table();
//         new_bin.insert("name", value("cli"));
//         new_bin.insert("path", value("src/cli.rs"));
//         bins.push(new_bin);
//     }

//     // Print the modified document
//     println!("{}", doc.to_string());

//     Ok(())
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish NOW](https://github.com/john-cd/rust_howto/issues/1159)
