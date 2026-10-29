# Purpose: Create and search GitHub issues from repository content.
# Usage: From bk/, run just gh <recipe>.
#
# TODO Reviewed ??-??-2026.

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")

set tempdir := "."

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

@login:
    gh auth login

# Create a GH issue with a title and an empty body
@issue title:
    gh issue create --title {{ quote(title) }} --body ''

# Return a list of GitHub issues with the same title
find_dupe_issues:
    #!/usr/bin/env bash
    set -euo pipefail
    IFS=$'\t\n'
    gh issue list -L 2000 --json title --jq '.[] | [.title]' | sed -E 's/\["(.*)(\(P[0-9]\))?"]/\1/g' | sort | uniq --repeated

# --template '{{range .}}{{.number .title}}{{"\n"}}{{end}}
# --jq '.[] | [.number, .title]'

# List TODO notes in tracked Markdown files; pass `apply` to create issues and link them.
create_issues_for_markdown mode="":
    bash ./todo_issues.sh {{ if mode == "apply" { "--apply" } else { "" } }} --kind md {{ quote(root + "/..") }}

# List TODO notes in tracked .rs files; pass `apply` to create issues and link them.
create_issues_for_code mode="":
    bash ./todo_issues.sh {{ if mode == "apply" { "--apply" } else { "" } }} --kind code {{ quote(root + "/..") }}

# List TODO notes in other tracked files (*.sh, *.toml, *.yaml...); pass `apply` to create issues.
create_issues_for_other mode="":
    bash ./todo_issues.sh {{ if mode == "apply" { "--apply" } else { "" } }} --kind other {{ quote(root + "/..") }}

# Run the todo_issues.sh fixtures (fake gh, no GitHub writes).
test:
    bash ./test_todo_issues.sh

# Preview refreshing source-linked issues (read-only); pass `apply` to edit titles and the generated source-link section.
refresh mode="":
    bash ./refresh_issues.sh {{ if mode == "apply" { "--apply" } else { "" } }} {{ quote(root + "/..") }}

# Run the refresh_issues.sh fixtures (fake gh, no GitHub access).
test_refresh:
    bash ./test_refresh_issues.sh
