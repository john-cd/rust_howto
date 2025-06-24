#![allow(dead_code)]
// ANCHOR: example
use std::thread;
use std::time::Duration;

use console::Emoji;
use console::Style;
use console::Term;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

// This function demonstrates the use of the `console` and `indicatif` crates
// to create a visually appealing command-line interface.
// It simulates the output of a compiler, showing different stages of the
// compilation process with a spinner and styled text.
// `console` is a library for Rust that provides access to
// terminal features, like styles and colors.
fn spinner() -> anyhow::Result<()> {
    // The terminal is abstracted through the `console::Term` type.
    let term = Term::stdout();
    term.clear_screen()?;
    term.write_line("Simulate the output of a compiler:")?;
    thread::sleep(Duration::from_millis(20));
    term.clear_line()?;
    // Print with styles / colors:
    println!(
        "Target: {}",
        console::style("x86_64-unknown-linux-gnu").cyan()
    );

    // `indicatif` offers progress bars and spinners.
    // Create a progress bar with an indeterminate length:
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .expect("Failed to set progress style"),
    );
    // Simulate different stages of the compilation process:
    let stages = vec![
        ("Parsing source files", 100),
        ("Type checking", 300),
        ("Optimizing code", 500),
        ("Generating bytecode", 100),
        ("Linking objects", 300),
        ("Finalizing build", 100),
    ];
    // Spawns a background thread to tick the progress bar:
    pb.enable_steady_tick(Duration::from_millis(50));
    for (stage, duration) in stages {
        pb.set_message(stage);
        thread::sleep(Duration::from_millis(duration));
    }
    pb.disable_steady_tick();
    pb.finish_with_message("Compilation complete!");
    // Print with style:
    let cyan = Style::new().cyan();
    // Print the location of the executable with a cyan style:
    println!("Executable: {}", cyan.apply_to("./target/debug/my_program"));
    // Use emojis:
    println!("{} Done!", Emoji("✨", ":-)"));
    Ok(())
}

fn main() -> anyhow::Result<()> {
    spinner()?;
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() {
    main().unwrap();
}
