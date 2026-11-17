//! Runs the module visibility example.
//!
//! TODO Reviewed ??-??-2026.

/// Resolve paths between modules and re-export a public module.
mod module_paths;
/// Visibility of items in modules.
mod module_visibility;

/// Runs the modules example.
fn main() {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("module-paths"));
    command = command.subcommand(clap::Command::new("module-visibility"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("module-paths", _)) => {
            module_paths::run();
        }
        Some(("module-visibility", _)) => {
            module_visibility::run();
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}

/// Provides module forms behavior for the modules component.
#[cfg(test)]
mod module_forms;
