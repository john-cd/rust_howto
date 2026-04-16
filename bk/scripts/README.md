# Scripts

The `scripts` folder contains [just][just~website]↗ modules (`mod.just` files within a folder) and [bash][bash~website]↗ shell scripts (`*.sh` files).

The main `justfile` for the book, located in the `bk` folder, imports these modules. They include recipes that execute shell commands, call bash scripts, or call tools written in Rust (source code in the `tools` folder, release binaries in `bin`).

Type `just` at the terminal to display a list of available modules and commands to manage the book.

Type `just <module_name>` to display available commands for a given module.

## Key

- `deps`: dependency management.
- `docker`: [Docker][docker~website]↗ commands.
- `docs`: code documentation generation.
- `gh`: Github commands.
- `toc`: main table of contents.
- `urls`: URLs to external websites.
- `utils`: utilities.

[bash~website]: https://gnu.org/software/bash
[docker~website]: https://docker.com
[just~website]: https://just.systems/man/en
