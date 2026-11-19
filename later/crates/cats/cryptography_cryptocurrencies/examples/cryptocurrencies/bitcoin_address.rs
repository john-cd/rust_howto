// TODO Reviewed ??-??-2026.

// ANCHOR: example
use bitcoin::Address;
use bitcoin::Network;
use bitcoin::address::NetworkUnchecked;

/// Public address fixture from the rust-bitcoin documentation.
const ADDRESS: &str = "32iVBEu4dxkUQk9dJbZUiBiQdmypcEyJRf";

/// Parses the fixture, checks its network and prints the output script.
///
/// # Errors
///
/// Returns an error if the address is malformed or incompatible with mainnet.
pub fn run() -> anyhow::Result<()> {
    let unchecked: Address<NetworkUnchecked> = ADDRESS.parse()?;
    let address = unchecked.require_network(Network::Bitcoin)?;
    println!(
        "Bitcoin output script: {}",
        address.script_pubkey().to_hex_string()
    );
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Rejects the mainnet address when a test network was requested.
    #[test]
    fn rejects_the_wrong_network() -> anyhow::Result<()> {
        let address: Address<NetworkUnchecked> = ADDRESS.parse()?;
        assert!(address.require_network(Network::Testnet).is_err());
        Ok(())
    }

    /// Produces the expected pay-to-script-hash output.
    #[test]
    fn outputs_p2sh_script() -> anyhow::Result<()> {
        let address = ADDRESS
            .parse::<Address<NetworkUnchecked>>()?
            .require_network(Network::Bitcoin)?;
        assert!(address.script_pubkey().is_p2sh());
        Ok(())
    }
}
