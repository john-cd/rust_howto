# Purpose: Provide shared Bash recipes for book tasks with the same behavior on all platforms.
# Usage: From bk/, run just bash <recipe>.
#

# Shared Bash-backed recipes for book tasks that have the same behavior on all platforms.

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := clean(source_directory() + "/../..")
book_root := replace(root, "\\", "/")
book_script := replace(root + "/scripts/book/build_book.sh", "\\", "/")

# Build the book through the shared script, including its book.toml fallback.
mdbook-build:
    echo "Build book in {{ book_root }}"
    "{{ book_script }}" "{{ book_root }}"

# Generate the sitemap using mdbook-utils from the book root.
sitemap:
    echo "Generate the sitemap"
    cd "{{ book_root }}" && mdbook-utils sitemap

# Copy static assets into the generated HTML directory.
copystatic:
    echo "Copy static files"
    cp "{{ book_root }}/static/"*.* "{{ book_root }}/book/html/"

# Remove the doctest cache without invoking OS-specific deletion syntax.
clean-doctest-cache:
    cd "{{ book_root }}" && rm -rf ./doctest_cache/
