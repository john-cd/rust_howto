#[cfg(feature = "oracle")]
mod diesel_oci;
#[cfg(feature = "oracle")]
mod oracle;
#[cfg(all(target_os = "linux", feature = "oracle"))]
mod sibyl;

#[cfg(feature = "oracle")]
#[allow(dead_code)]
pub(crate) static ENV_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn main() {}
