# Purpose: Find files that are not referenced by the book source.
# Usage: Run with bash bk/crates/cats/development_tools_documentation/examples/mdbook_utils/find_unreferenced_files.sh from the example workspace, as described in the book.
#
mdbook-utils markdown identify-files-not-in-summary --markdown-dir ./src
mdbook-utils markdown identify-unused-rs-examples --markdown-dir ./src --target-dir ./crates
