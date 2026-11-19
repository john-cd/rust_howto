//! Runs the emulator example.
//!
//! TODO Reviewed ??-??-2026.

/// Parses an Intel HEX data record and its end-of-file record.
mod intel_hex;
/// A simple `PolkaVM` host example.
mod polkavm_host;
/// A two-register virtual machine with a bounded fetch/decode/execute loop.
mod register_vm;

/// Runs the emulators example.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut command = clap::Command::new("examples");
    command = command.subcommand(clap::Command::new("register-vm"));
    command = command.subcommand(
        clap::Command::new("polkavm-host").arg(
            clap::Arg::new("guest")
                .value_name("GUEST")
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .help("Use this guest blob instead of the bundled example"),
        ),
    );
    command = command.subcommand(clap::Command::new("intel-hex"));
    let matches = command
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();
    match matches.subcommand() {
        Some(("register-vm", _)) => {
            register_vm::run()?;
            Ok(())
        }
        Some(("polkavm-host", matches)) => {
            polkavm_host::run(
                matches
                    .get_one::<std::path::PathBuf>("guest")
                    .map(std::path::PathBuf::as_path),
            )?;
            Ok(())
        }
        Some(("intel-hex", _)) => {
            intel_hex::run()?;
            Ok(())
        }
        _ => unreachable!("Clap requires one of the declared subcommands"),
    }
}
