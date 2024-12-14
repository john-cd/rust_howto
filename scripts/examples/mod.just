set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
  @just --list examples --unsorted --justfile {{justfile()}}
#@just --choose

## ---- CODE EXAMPLE MANAGEMENT -----------------------------------

# List examples (.rs files) in `deps/tests` that are not included in the book markdown.
[unix]
list_not_used_in_book:
  ./list_examples_not_used_in_book.sh
# The script matches e.g. {{#include ../../../deps/tests/cats/development_tools_debugging/type_name_of_val.rs:example}}
# and extracts the file names
# then compare to the list of test files in `deps`
# A few files e.g. `main.rs` and `mod.rs` are not true examples and should not be included into the book.

# List examples (.rs files) in `deps/tests` that were not included into a module somehow and that are therefore not tested
[unix]
list_not_tested:
  #! /bin/bash
  root="{{justfile_directory()}}/"
  comm -1 -3 <(find "${root}deps/tests" -type f \( -name "main.rs" -o -name "mod.rs" \) -exec sed -nE 's/^mod (\w*)\s*?;/\1/pg' {} \; | sort -u) \
          <(find "${root}deps/tests" -mindepth 2 -type f -name "*.rs" -exec basename -s '.rs' {} \; | sed '/main/d; /mod/d' | sort -u)
  echo "DONE"

# Convert {{#example <name>}} placeholders into ```rust... {#include ...}``` blocks and create the necessary code stubs (in subfolders of deps/tests/)
[unix]
convert_example_placeholders:
  ./convert_example_placeholders.sh
