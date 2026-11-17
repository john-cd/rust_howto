<!-- TODO Reviewed ??-??-2026. -->

# OS

This crate contains code examples used by the book's **Operating Systems** category. It demonstrates integration with `anyhow`, `clap`, `ctrlc`, `duct`, `libc`, `nix`, `regex`, `shlex`, `tokio`, `wait-timeout`, `which`, `windows-sys`, and `xshell`.

## Examples Covered

- **External Commands** (`examples/external_commands/`): Demonstrates concepts like `continuous`, `duct_example`, `error_file`, `piped`, `process_output`, `read_env_variable`, `send_input`, `shlex_example`, `which`, `xshell_example`.
- **Low Level System Calls** (`examples/low_level_system_calls/`): Demonstrates concepts like `libc`.
- **Process Management** (`examples/process_management/`): Demonstrates concepts like `job_object`, `kill_guard`, `process_group`, `timeout_kill`.
- **Signals** (`examples/signals/`): Demonstrates concepts like `ctrlc_example`, `tokio_signal`.

## Usage

To check or run examples in this crate:

```console
cargo check --package os --examples
cargo test --package os
```

Code examples correspond to chapters in `bk/src/categories/os` or related book sections.
