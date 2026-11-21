# Purpose: Inspect and create book code examples and check their test coverage.
# Usage: From bk/, run just examples <recipe>.
#
# TODO Reviewed ??-??-2026.

#set shell := ["bash", "-uc"]
set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

[unix]
set script-interpreter := ["bash"]

[windows]
set script-interpreter := ["C:/Program Files/Git/bin/bash.exe"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")

# List the available recipes and task modules.
[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- CODE EXAMPLE MANAGEMENT -----------------------------------

# List examples (.rs files) in `crates/**/tests` that may not be included in the book markdown.
@list_not_used_in_book:
    cargo run --manifest-path "{{ root }}/../tools/Cargo.toml" --locked -p example-integrity -- --repo-root "{{ root }}/.." unused-examples --book-root bk

# The script matches e.g. {{#include ../../../crates/cats/development_tools_debugging/examples/<...>/type_name_of_val.rs:example}}
# and extracts the file names, then compare to the list of test files
# A few files e.g. `main.rs` and `mod.rs` are not true examples and should not be included into the book.

# List examples (.rs files) in `crates/**/tests` that were not included into a module somehow and that are therefore not tested
[script]
list_not_tested:
    root="{{ root }}/"
    comm -1 -3 <(find "${root}crates" -type f \( -name "*.rs" \) -exec sed -nE 's/^mod (\w*)\s*?;/\1/pg' {} \; | sort -u) \
          <(find "${root}crates" -type f -wholename "*/examples/*/*.rs" -exec basename -s '.rs' {} \; | sed '/main/d; /mod/d; /lib/d; /cat/d; /lang/d; /topic/d; /build/d' | sort -u)
    echo "DONE"

# Convert {{#example <name>}} placeholders into ```rust... {#include ...}``` blocks and create the necessary code stubs (in subfolders of crates)
@convert_example_placeholders:
    ./convert_example_placeholders.sh {{ root }}

# Create a new cat crate for a given crates.io category (e.g., `just create algorithms`)
@create cat:
    ./create.sh {{ root }} {{ cat }}
