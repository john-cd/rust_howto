// ANCHOR: example
use toml::Value;
use toml::de::Error;

fn main() -> Result<(), Error> {
    // Note the use of a raw string,
    // so that there is no need to escape the inner double quotes
    let toml_content = r#"
          [package]
          name = "your_package"
          version = "0.1.0"
          authors = ["You! <you@example.org>"]

          [dependencies]
          serde = "1.0"
          "#;

    let package_info: Value = toml::from_str(toml_content)?;

    assert_eq!(package_info["dependencies"]["serde"].as_str(), Some("1.0"));
    assert_eq!(
        package_info["package"]["name"].as_str(),
        Some("your_package")
    );

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [toml: print (P0)](https://github.com/john-cd/rust_howto/issues/163)
