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

# Refresh GitHub issues
[confirm]
refresh:
    #!/usr/bin/env bash
    set -euo pipefail
    IFS=$'\n\t'
    repo=$( realpath "{{ root }}/../" ) # directory of the repo
    for file in $( find "${repo}" -mindepth 1 -maxdepth 1 \( -name .git -o -name target -o -name bin \) -prune -o \( -type d -print0 \) | find -files0-from - -type f -not -path "*/book/*" \( -name "*.rs" -o -name "*.md" -o -name "*.toml" -o -name "*.yaml" -o -name "*.yml" \) )
    do
      rel=$(realpath --relative-to=${repo} ${file})
      #echo "> ${rel}"
      for str in $( sed -En 's~^\s*\[([^]]*?)\]\(https://github.com/john-cd/rust_howto/issues/([0-9]+)\)~\1@\2@~p' "${file}" )
      do
        todo=$( echo "$str" | cut -d@ -f1 )
        id=$( echo "$str" | cut -d@ -f2 )
        existing_title=$(gh issue view "${id}" --json title --jq '.title')
        sleep 1
        if [ "$existing_title" != "${rel}: ${todo:-fix}" ];
        then
          echo "---"
          echo "id: ${id}"
          echo "previous title: $existing_title"
          echo "title: ${rel}: ${todo:-fix}"
          echo "body: [$rel](https://github.com/john-cd/rust_howto/blob/develop/${rel})"
          gh issue edit "${id}" --title "${rel}: ${todo:-fix}" --body "[$rel](https://github.com/john-cd/rust_howto/blob/develop/${rel})" --add-assignee "@me" --add-label "auto"
          sleep 2
        fi
      done
    done
