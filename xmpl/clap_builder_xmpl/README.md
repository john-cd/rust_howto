# Clap Builder Example

This example demonstrates how to build a complete command-line interface (CLI) using [`clap`](https://docs.rs/clap)'s builder API. It organizes the CLI logic across multiple modules and showcases the following features:

- **Subcommands**: `open` (accepts one or more file paths), `query` (accepts a sequence of query words), and `test` (a hidden command).
- **Global flags**: `--verbose` / `-v` (can be repeated for increasing verbosity) and `--config` / `-c` (accepts a file path), both readable from environment variables (`TOOL_VERBOSE` and `TOOL_CONFIG_FILE`).
- **Custom help styling**: colors and formatting applied to the help output.
- **Inferred subcommands**: partial name matches are accepted (e.g., `te` matches `test`).
- **Builder pattern**: a `Config` struct built with a `ConfigBuilder`, separating CLI parsing from configuration.

## Usage

```sh
cargo run -- --help
cargo run -- -vv open file1.csv file2.csv
cargo run -- --config config.toml query SELECT col FROM tbl
cargo run -- -c config.toml test
```
