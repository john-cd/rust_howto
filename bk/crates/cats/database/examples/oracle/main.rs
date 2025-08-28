#[cfg(feature = "oracle")]
mod diesel_oci;
#[cfg(feature = "oracle")]
mod oracle;
#[cfg(all(target_os = "linux", feature = "oracle"))]
mod sibyl;

fn main() {}
