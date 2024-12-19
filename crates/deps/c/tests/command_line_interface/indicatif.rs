// ANCHOR: example
use std::thread;
use std::time::Duration;

use console::Emoji;
use console::Style;
use console::Term;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;

fn spinner() -> anyhow::Result<()> {
    // `console` is a library for Rust that provides access to
    // terminal features, like styles and colors.
    // The terminal is abstracted through the `console::Term` type.
    let term = Term::stdout();
    term.clear_screen()?;
    term.write_line("Simulate the output of a compiler:")?;
    thread::sleep(Duration::from_millis(20));
    term.clear_line()?;
    // Print with styles / colors
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
    // Simulate different stages of the compilation process
    let stages = vec![
        ("Parsing source files", 1),
        ("Type checking", 3),
        ("Optimizing code", 2),
        ("Generating bytecode", 1),
        ("Linking objects", 1),
        ("Finalizing build", 1),
    ];
    // Spawns a background thread to tick the progress bar
    pb.enable_steady_tick(Duration::from_millis(100));
    for (stage, duration) in stages {
        pb.set_message(stage);
        thread::sleep(Duration::from_secs(duration));
    }
    pb.disable_steady_tick();
    pb.finish_with_message("Compilation complete!");
    // Print with style
    let cyan = Style::new().cyan();
    println!("Executable: {}", cyan.apply_to("./target/debug/my_program"));
    // Use emojis
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
