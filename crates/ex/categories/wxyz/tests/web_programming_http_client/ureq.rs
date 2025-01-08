// ANCHOR: example
use ureq::Error;

fn main() -> Result<(), Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let response: String = ureq::get(url).call()?.into_string()?;

    println!("Response: {}", response);
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> Result<(), Error> {
    main()?;
    Ok(())
}
// [P1](https://github.com/john-cd/rust_howto/issues/862)
