#![allow(dead_code)]
// ANCHOR: example
//! Build a platform TLS connector with `native-tls`.
//!
//! This example constructs a `TlsConnector` and prints a note about when to
//! use it with a TCP stream.

use native_tls::TlsConnector;

fn main() -> anyhow::Result<()> {
    let _connector = TlsConnector::builder().build()?;

    println!(
        "Built a native-tls connector. Use it with a TCP stream when you need a platform TLS implementation."
    );
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
    // [finish](https://github.com/john-cd/rust_howto/issues/696)
}
