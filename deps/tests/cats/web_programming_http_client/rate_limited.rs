// ANCHOR: example
// use std::thread;
// use std::time::Duration;
// use std::time::UNIX_EPOCH;

// use anyhow::anyhow;
// use anyhow::Result;
// use reqwest::StatusCode;

// // TODO fix interaction with https://docs.github.com/en/rest?apiVersion=2022-11-28

// fn main() -> Result<()> {
//     let url = "https://api.github.com/users/john-cd";
//     let client = reqwest::blocking::Client::new();

//     loop {
//         let response = client.get(url).send()?;
//         let hdrs = response.headers();

//         let rate_limit: usize =
// hdrs.get("x-ratelimit-limit").ok_or_else( || {
// anyhow!("response doesn't include the expected x-ratelimit-limit
// header") }         )?.to_str()?.parse()?;

//         let rate_remaining: usize =
// hdrs.get("x-ratelimit-remaining")         .ok_or_else(|| {
// anyhow!("response doesn't include the expected
// x-ratelimit-remaining header") })?.to_str()?.parse()?;

//         let rate_reset_at: u64 = hdrs.get("x-ratelimit-reset")
//         .ok_or_else(|| { anyhow!("response doesn't include the
// expected x-ratelimit-reset header") })?.to_str()?.parse()?;

//         let rate_reset_within =
//             Duration::from_secs(rate_reset_at) -
// UNIX_EPOCH.elapsed()?;

//         if response.status() == StatusCode::FORBIDDEN &&
// rate_remaining == 0 {             println!("Sleeping for {}
// seconds.", rate_reset_within.as_secs());
// thread::sleep(rate_reset_within);             continue;
//         } else {
//             println!(
//                 "Rate limit is currently {}/{}, the reset of this
// limit will be within {} seconds.",                 rate_remaining,
//                 rate_limit,
//                 rate_reset_within.as_secs(),
//             );
//             break;
//         }
//     }
//     Ok(())
// }

fn main() -> anyhow::Result<()> {
    Ok(())
}

// ANCHOR_END: example

// requires network access
#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
