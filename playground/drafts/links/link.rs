use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::trace;
use tracing::warn;

pub struct Link {
    url: url::Url,
}

// TODO
pub fn check_links(content: &str, check_external_links: bool) -> Vec<Link> {
    // if check_external_links {}
    Vec::new()
}

// TODO
fn check_external_links(content: &str) -> Vec<Link> {
    todo!();
    // Vec::new()
}
