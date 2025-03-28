// ANCHOR: example
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
struct MyConfig {
    version: u8,
    api_key: String,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        Self {
            version: 0,
            api_key: "".into(),
        }
    }
}

fn main() -> Result<(), confy::ConfyError> {
    let cfg: MyConfig = confy::load("my-app-name", None)?;
    // confy::store("my-app-name", None, cfg)?;
    println!("{:?}", cfg);
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [review](https://github.com/john-cd/rust_howto/issues/148)
