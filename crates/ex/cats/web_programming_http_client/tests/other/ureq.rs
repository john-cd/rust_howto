// ANCHOR: example

fn main() -> anyhow::Result<()> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let response: String =
        ureq::get(url).call()?.body_mut().read_to_string()?;

    println!("Response: {}", response);
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [P1](https://github.com/john-cd/rust_howto/issues/862)
