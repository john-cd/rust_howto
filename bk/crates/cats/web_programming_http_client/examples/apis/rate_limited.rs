#![allow(dead_code)]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This example demonstrates how to handle rate limiting when interacting
// //! with an API.
// //!
// //! It specifically targets the GitHub API, which provides rate limit
// //! information in its headers. The code checks for the `x-ratelimit-limit`,
// //! `x-ratelimit-remaining`, and `x-ratelimit-reset` headers. If the rate
// //! limit is exceeded, the program will sleep until the reset time.
// use std::thread;
// use std::time::Duration;
// use std::time::UNIX_EPOCH;

// use anyhow::Result;
// use anyhow::anyhow;
// use reqwest::StatusCode;

// /// Demonstrates rate limit handling.
// ///
// /// It fetches user information from the GitHub API and checks for rate
// /// limit headers. If the rate limit is exceeded, it sleeps until the reset
// /// time.
// fn main() -> Result<()> {
//     let url = "https://api.github.com/users/john-cd";
//     let client = reqwest::blocking::Client::new();

//     loop {
//         let response = client.get(url).send()?;
//         let hdrs = response.headers();

//         let rate_limit: usize = hdrs
//             .get("x-ratelimit-limit")
//             .ok_or_else(|| {
//                 anyhow!(
//              "Response doesn't include the expected x-ratelimit-limit header"
//                 )
//             })?
//             .to_str()?
//             .parse()?;

//         let rate_remaining: usize = hdrs
//             .get("x-ratelimit-remaining")
//             .ok_or_else(|| {
//                 anyhow!(
//   "Response doesn't include the expected x-ratelimit-remaining header"
//                )
//             })?
//             .to_str()?
//             .parse()?;

//         let rate_reset_at: u64 = hdrs
//             .get("x-ratelimit-reset")
//             .ok_or_else(|| {
//                 anyhow!(
//        "Response doesn't include the expected x-ratelimit-reset header"
//                 )
//             })?
//             .to_str()?
//             .parse()?;

//         let rate_reset_within =
//             Duration::from_secs(rate_reset_at) - UNIX_EPOCH.elapsed()?;

//         if response.status() == StatusCode::FORBIDDEN && rate_remaining == 0
// {             println!("Sleeping for {} seconds.",
// rate_reset_within.as_secs());             thread::sleep(rate_reset_within);
//             continue;
//         } else {
//             println!(
//                 "Rate limit is currently {}/{}, the reset of this limit will
// be within {} seconds.",                 rate_remaining,
//                 rate_limit,
//                 rate_reset_within.as_secs(),
//             );
//             break;
//         }
//     }
//     Ok(())
// }

// #[test]
// fn require_network() -> anyhow::Result<()> {
//     let res = main();
//     println!("{res:?}");
//     res?;
//     Ok(())
// }
// // [fix interaction with https://docs.github.com/en/rest?apiVersion=2022-11-28 NOW](https://github.com/john-cd/rust_howto/issues/177)
