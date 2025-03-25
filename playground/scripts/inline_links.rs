#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
clap = { version = "4.2", features = ["derive"] }
---

// https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#script

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short, long, help = "Path to config")]
    config: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
