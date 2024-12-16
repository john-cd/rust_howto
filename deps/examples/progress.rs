use std::thread;
use std::time::Duration;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

fn spinner() -> anyhow::Result<()> {
    let term = console::Term::stdout();
    term.write_line("Simulate the output of a compiler:")?;
    thread::sleep(Duration::from_millis(20));
    term.clear_line()?;

    // Create a progress bar with an indeterminate  length initially
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
    for (stage, duration) in stages {
        pb.set_message(stage);
        pb.enable_steady_tick(Duration::from_millis(100));
        thread::sleep(Duration::from_secs(duration));
        pb.disable_steady_tick();
    }
    pb.finish_with_message("Compilation complete!");
    // Print with styles / colors
    use console::style;
    println!("Target: {}", style("x86_64-unknown-linux-gnu").cyan());
    let cyan = console::Style::new().cyan();
    println!("Executable: {}", cyan.apply_to("./target/debug/my_program"));
    Ok(())
}

fn progress() {
    // Create a new progress bar with a length of 100
    // This progress bar by default draws directly to stderr.
    // If a non terminal is detected the progress bar will be completely hidden.
    let pb = ProgressBar::new(100);

    // Set a custom style for the progress bar
    pb.set_style(ProgressStyle::default_bar()
        // 40 characters wide and has cyan as primary style color and blue as alternative style color.
        .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("=>."));

    // Simulate some work
    for _ in 0..100 {
        // Update the progress bar
        pb.inc(1);
        // Simulate work by sleeping for a short duration
        thread::sleep(Duration::from_millis(50));
    }
    // Finish the progress bar
    pb.finish_with_message("done"); // or .finish();
}

fn iterate() {
    use indicatif::ProgressIterator;
    for _ in (0..100).progress() {
        // ...
        thread::sleep(Duration::from_millis(10));
    }
}

fn main() -> anyhow::Result<()> {
    spinner()?;
    progress();
    iterate();
    Ok(())
}
// TODO P0 improve
