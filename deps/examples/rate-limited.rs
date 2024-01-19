// use std::thread;
// use std::time::Duration;
// use std::time::UNIX_EPOCH;

use anyhow::Result;
// use reqwest::StatusCode;

// header! { (XRateLimitLimit, "X-RateLimit-Limit") => [usize] }
// header! { (XRateLimitRemaining, "X-RateLimit-Remaining") => [usize]
// } header! { (XRateLimitReset, "X-RateLimit-Reset") => [u64] }

fn main() -> Result<()> {
    // loop {
    //     let url = "https://api.github.com/users/rust-lang-nursery ";
    //     let client = reqwest::Client::new();
    //     let response = client.get(url).send()?;

    //     let rate_limit =
    // response.headers().get::<XRateLimitLimit>().ok_or(
    //         "response doesn't include the expected X-RateLimit-Limit
    // header",     )?;

    //     let rate_remaining =
    // response.headers().get::<XRateLimitRemaining>().ok_or(
    //         "response doesn't include the expected
    // X-RateLimit-Remaining header",     )?;

    //     let rate_reset_at =
    // response.headers().get::<XRateLimitReset>().ok_or(
    //         "response doesn't include the expected X-RateLimit-Reset
    // header",     )?;

    //     let rate_reset_within =
    //         Duration::from_secs(**rate_reset_at) -
    // UNIX_EPOCH.elapsed()?;

    //     if response.status() == StatusCode::Forbidden &&
    // **rate_remaining == 0 {         println!("Sleeping for {}
    // seconds.", rate_reset_within.as_secs());
    //         thread::sleep(rate_reset_within);
    //         return main();
    //     } else {
    //         println!(
    //             "Rate limit is currently {}/{}, the reset of this limit
    // will be within {} seconds.",             **rate_remaining,
    //             **rate_limit,
    //             rate_reset_within.as_secs(),
    //         );
    //         break;
    //     }
    // }
    Ok(())
}
