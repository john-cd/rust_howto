use std::fs::File;
use std::io::Read;

use anyhow::anyhow;
use anyhow::Result;

fn read_uptime() -> Result<u64> {
    let mut uptime = String::new();
    File::open("/proc/uptime")?.read_to_string(&mut uptime)?;

    Ok(uptime
        .split('.')
        .next()
        .ok_or(anyhow!("Cannot parse uptime data"))?
        .parse()?)
}

#[test]
fn test() {
    match read_uptime() {
        Ok(uptime) => println!("uptime: {} seconds", uptime),
        Err(err) => eprintln!("error: {}", err),
    };
}
