//! Runs the immediate mode GUI example.
//!
//! TODO Reviewed ??-??-2026.

/// Simple example of a GUI application with `egui`.
mod egui;
/// Runs an `egui` UI for one frame without a window, a GPU or a display.
mod egui_headless;
/// Finds a button by its label and clicks it with `egui_kittest`.
mod egui_kittest;
/// A line plot and a bar chart with `egui_plot`.
mod egui_plot;
/// A virtualized data table with `egui_extras`.
mod egui_table;

/// Runs the immediate mode gui example.
fn main() -> Result<(), eframe::Error> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("egui-headless"));
    command = command.subcommand(clap::Command::new("egui-kittest"));
    command = command.subcommand(clap::Command::new("egui-plot"));
    command = command.subcommand(clap::Command::new("egui-table"));
    command = command.subcommand(clap::Command::new("egui"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("egui-headless", _)) => {
            egui_headless::run();
            Ok(())
        }
        Some(("egui-kittest", _)) => {
            egui_kittest::run();
            Ok(())
        }
        Some(("egui-plot", _)) => {
            egui_plot::run();
            Ok(())
        }
        Some(("egui-table", _)) => {
            egui_table::run();
            Ok(())
        }
        Some(("egui", _)) => {
            egui::run()?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
