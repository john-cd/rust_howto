// ANCHOR: example
use serde_json::json;
use serde_json::Error;
use serde_json::Value;

fn main() -> Result<(), Error> {
    let j = r#"{
                 "userid": 103609,
                 "verified": true,
                 "access_privileges": [
                   "user",
                   "admin"
                 ]
               }"#;

    let parsed: Value = serde_json::from_str(j)?;

    let expected = json!({
        "userid": 103609,
        "verified": true,
        "access_privileges": [
            "user",
            "admin"
        ]
    });

    assert_eq!(parsed, expected);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
