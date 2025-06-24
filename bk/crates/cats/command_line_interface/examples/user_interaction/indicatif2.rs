#![allow(dead_code)]
// ANCHOR: example
use std::thread;
use std::time::Duration;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

fn progress() {
    // Create a new progress bar with a length of 100
    // This progress bar by default draws directly to stderr.
    // If a non terminal is detected,
    // the progress bar will be completely hidden.
    let pb = ProgressBar::new(100);

    // Set a custom style for the progress bar
    pb.set_style(ProgressStyle::default_bar()
        // 40 characters wide and has cyan as primary style color and blue as alternative style color.
        .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("=>."));

    for _ in 0..100 {
        // Update the progress bar
        pb.inc(1);
        // Simulate work by sleeping for a short duration
        thread::sleep(Duration::from_millis(50));
    }
    // Finish the progress bar
    pb.finish_with_message("done"); // or use .finish();

    // Alternatively, you could wrap an iterator with a progress bar:
    // use indicatif::ProgressIterator;
    // for _ in (0..100).progress() {
    //     thread::sleep(Duration::from_millis(50));
    // }
}

fn main() -> anyhow::Result<()> {
    progress();
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() {
    main().unwrap();
}
