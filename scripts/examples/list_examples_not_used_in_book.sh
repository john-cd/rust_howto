#! /bin/bash

# List examples (.rs files) in `deps/tests` that are not included in the book markdown.
# Usage: ./scripts/examples/list_examples_not_used_in_book.sh

grep -Proh '\{\{#include .+?\.rs(:.+?)?\}\}' ./src ./drafts | sed -E 's~\{\{#include .+/([._a-zA-Z0-9]+?\.rs)(:.+?)?\}\}~\1~' | sort -u > /tmp/examples_in_markdown.txt
find ./deps/tests -type f -name "*.rs" -exec basename {} \; | sed '/main.rs/d; /mod.rs/d' | sort -u > /tmp/examples_in_deps.txt
comm -13 /tmp/examples_in_markdown.txt /tmp/examples_in_deps.txt

# The script matches e.g. {{#include ../../../deps/tests/cats/development_tools_debugging/type_name_of_val.rs:example}} and extracts the file names
# then compare to the list of test files in deps
# A few files e.g. `main.rs` and `mod.rs` are not true examples and should not be included into the book.
