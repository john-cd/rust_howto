#!/usr/bin/env bash
# Create a new cat crate for a given crates.io category.
#
# Usage: ./scripts/examples/create.sh <root folder> <category name>
#
# The root folder should be the `bk` directory.
# The category name should match the crates.io category slug (e.g., algorithms, asynchronous).

set -euo pipefail

if [[ $# -ne 2 ]]; then
    echo "Usage: $0 <root folder> <category name>" >&2
    exit 1
fi

root="$(realpath "$1")/"
cat="$2"
path="${root}crates/cats/${cat}"

if [ -d "${path}" ]; then
    echo "Error: ${path} already exists!" >&2
    exit 1
fi

echo ">> Creating crate: ${path}"

mkdir -p "${path}/examples"
mkdir -p "${path}/temp"
touch "${path}/temp/.gitkeep"

cat > "${path}/.gitignore" <<-EOF
	temp/*
	!temp/.gitkeep
	target/
EOF

license_src="$(find "${root}crates/cats" -maxdepth 2 -name "LICENSE" | head -1)"
if [[ -z "${license_src}" ]]; then
    echo "Error: no LICENSE file found under ${root}crates/cats/ to copy." >&2
    exit 1
fi
cp "${license_src}" "${path}/LICENSE"

cat > "${path}/README.md" <<-EOF
	# README

	Code examples are found under \`examples/<chapter_name>\`, where \`<chapter_name>\` is the name of the \`.md\` file in \`bk/<...>/categories/${cat}\` that includes these examples.
EOF

cat > "${path}/Cargo.toml" <<-EOF
	[package]
	name = "${cat}"
	version.workspace = true
	authors.workspace = true
	edition.workspace = true
	rust-version.workspace = true
	description = "Book code examples and their crate dependencies - ${cat}"
	documentation.workspace = true
	homepage.workspace = true
	repository.workspace = true

	keywords.workspace = true
	categories = ["${cat}"]
	publish.workspace = true
	autolib = false

	[dependencies]
EOF

echo "DONE: ${path} created."
echo "NOTE: The crate is automatically included in the workspace via 'cats/*' in crates/Cargo.toml."
