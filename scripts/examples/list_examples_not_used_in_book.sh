#! /bin/bash
set -eu

# List examples (.rs files) in `tests` that are not included in the book markdown.
# Usage: /code/scripts/examples/list_examples_not_used_in_book.sh

root="/code/"

grep -Proh '\{\{#include .+?\.rs(:.+?)?\}\}' "${root}src" "${root}drafts" | sed -E 's~\{\{#include [./]*([^.]+/[._a-zA-Z0-9]+?\.rs)(:.+?)?\}\}~\1~' | sort -u > /tmp/examples_in_markdown.txt
find "${root}crates/ex/" -type f -name "*.rs" -exec realpath --relative-to=${root} {} \; | sed '/lib.rs/d; /main.rs/d; /mod.rs/d; /build.rs/d; /cat.rs/d; /lang.rs/d; /std.rs/d; /topic.rs/d' | sort -u > /tmp/examples_in_crates_ex.txt
comm -13 /tmp/examples_in_markdown.txt /tmp/examples_in_crates_ex.txt

# The script matches e.g. {{#include ../../../crates/ex/categories/d/tests/development_tools_debugging/type_name_of_val.rs:example}} and extracts the file names
# then compare to the list of test files in `crates/ex`
# A few files e.g. `main.rs` and `mod.rs` are not true examples and should not be included into the book.

echo "DONE"
