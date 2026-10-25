mdbook-utils markdown extract-code-examples --markdown-dir ./drafts --target-dir ./code
mdbook-utils markdown replace-code-examples-by-includes --markdown-dir ./drafts --target-dir ./code
mdbook-utils markdown replace-includes-by-contents --markdown-dir ./src
mdbook-utils markdown remove-includes --markdown-dir ./book/markdown
