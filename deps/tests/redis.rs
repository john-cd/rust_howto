use anyhow::Result;
use redis::Commands;

fn fetch_an_integer() -> redis::RedisResult<isize> {
    // `open` does not actually open a connection yet but it does perform some
    // basic checks on the URL that might make the operation fail.
    let client = redis::Client::open("redis://127.0.0.1/")?;
    // actually connect to redis
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _: () = con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}

fn main() -> Result<()> {
    let my_int = fetch_an_integer()?;
    println!("{}", my_int);
    Ok(())
}

// TODO
#[ignore]
#[test]
fn test() -> Result<()> {
    let res = main();
    println!("{:?}", res);
    res
}
