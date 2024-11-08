use anyhow::Result;
use crates_io_api::CrateResponse;
use tracing::warn;

/// Returns information for a crate, given its name
pub fn get_info_for_crate(crate_name: &str) -> Result<CrateResponse> {
    let client = super::get_client()?;
    warn!("Calling crates.io API for {crate_name}");
    let crt = client.get_crate(crate_name)?;
    Ok(crt)
}