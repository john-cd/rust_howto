#set shell := ["bash", "-uc"]
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := justfile_directory()

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- CODE EXAMPLE MANAGEMENT -----------------------------------

# List examples (.rs files) in `crates/**/tests` that may not be included in the book markdown.
[unix]
@list_not_used_in_book:
    ./list_examples_not_used_in_book.sh {{ quote(justfile_directory()) }}
# The script matches e.g. {{#include ../../../crates/cats/development_tools_debugging/tests/<...>/type_name_of_val.rs:example}}
# and extracts the file names, then compare to the list of test files
# A few files e.g. `main.rs` and `mod.rs` are not true examples and should not be included into the book.

# List examples (.rs files) in `crates/**/tests` that were not included into a module somehow and that are therefore not tested
[unix]
list_not_tested:
    #! /bin/bash
    root="{{root}}/"
    comm -1 -3 <(find "${root}crates/ex" -type f \( -name "*.rs" \) -exec sed -nE 's/^mod (\w*)\s*?;/\1/pg' {} \; | sort -u) \
          <(find "${root}crates/ex" -type f -wholename "*/tests/*/*.rs" -exec basename -s '.rs' {} \; | sed '/main/d; /mod/d; /lib/d; /cat/d; /lang/d; /topic/d; /build/d' | sort -u)
    echo "DONE"

# Convert {{#example <name>}} placeholders into ```rust... {#include ...}``` blocks and create the necessary code stubs (in subfolders of crates/ex)
[unix]
@convert_example_placeholders:
    ./convert_example_placeholders.sh {{root}}
