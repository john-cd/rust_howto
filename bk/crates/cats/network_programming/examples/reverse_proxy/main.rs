mod ngrok;
// Linux is Pingora's tier 1 environment and main focus.
#[cfg(target_family = "unix")]
mod pingora;

fn main() {}
