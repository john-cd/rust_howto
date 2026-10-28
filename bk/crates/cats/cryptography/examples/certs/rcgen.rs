// ANCHOR: example
//! Generate a self-signed certificate for local development with `rcgen`.

use rcgen::generate_simple_self_signed;

/// Creates and prints a certificate for the `localhost` name.
fn main() -> anyhow::Result<()> {
    let certified_key =
        generate_simple_self_signed(vec!["localhost".to_owned()])?;

    // Print the public certificate only; do not print or commit the private
    // key.
    println!("{}", certified_key.cert.pem());
    Ok(())
}
// ANCHOR_END: example

/// Runs the self-signed certificate example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Generates and prints a self-signed localhost certificate.
    #[test]
    fn generates_localhost_certificate() -> anyhow::Result<()> {
        main()
    }
}
