mod cli;
use rust_rate;

use tracing_subscriber::prelude::*;

fn main() {
    // Init logging
    tracing_subscriber::fmt().init();
}
