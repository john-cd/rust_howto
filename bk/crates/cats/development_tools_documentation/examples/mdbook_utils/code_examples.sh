# Purpose: Extract or inspect code examples with mdbook-utils.
# Usage: Run with bash bk/crates/cats/development_tools_documentation/examples/mdbook_utils/code_examples.sh from the example workspace, as described in the book.
#
mdbook-utils markdown extract-code-examples --markdown-dir ./drafts --target-dir ./code
mdbook-utils markdown replace-code-examples-by-includes --markdown-dir ./drafts --target-dir ./code
mdbook-utils markdown replace-includes-by-contents --markdown-dir ./src
mdbook-utils markdown remove-includes --markdown-dir ./book/markdown
