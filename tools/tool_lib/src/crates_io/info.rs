use anyhow::Result;
use crates_io_api::CrateResponse;
use tracing::debug;

/// Returns information for a crate, given its name
pub fn get_info_for_crate(crate_name: &str) -> Result<CrateResponse> {
    let client = super::get_client()?;
    debug!("Calling crates.io API for {crate_name}");
    let crt = client.get_crate(crate_name)?;
    Ok(crt)
}
// [unit tests](https://github.com/john-cd/rust_howto/issues/1358)
