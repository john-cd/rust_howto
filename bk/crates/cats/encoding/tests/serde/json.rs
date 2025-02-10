// ANCHOR: example
use serde_json::Error;
use serde_json::Value;
use serde_json::json;

fn main() -> Result<(), Error> {
    let j: &str = r#"{
                 "userid": 103609,
                 "verified": true,
                 "access_privileges": [
                   "user",
                   "admin"
                 ]
               }"#;

    let parsed: Value = serde_json::from_str(j)?;

    let expected: Value = json!({
        "userid": 103609,
        "verified": true,
        "access_privileges": [
            "user",
            "admin"
        ]
    });
    println!("{}", expected);
    assert_eq!(parsed, expected);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
