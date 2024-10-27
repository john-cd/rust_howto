## Redis

## Do something with Redis

[![redis][c-redis-badge]][c-redis]
[![redis-crates.io][c-redis-crates.io-badge]][c-redis-crates.io]
[![redis-github][c-redis-github-badge]][c-redis-github]
[![redis-lib.rs][c-redis-lib.rs-badge]][c-redis-lib.rs]

Redis-rs is a high level redis library for Rust. It provides convenient access to all Redis functionality through a very flexible but low-level API. It uses a customizable type conversion trait so that any operation can return results in just the type you are expecting. This makes for a very pleasant development experience.

```rust
use redis::Commands;

fn fetch_an_integer() -> redis::RedisResult<isize> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _: () = con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
