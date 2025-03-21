set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

@login:
  gh auth login

# Create a GH issue with a title and an empty body
@issue title:
  gh issue create --title {{title}} --body ''

# Return a list of GitHub issues with the same title
[unix]
find_dupe_issues:
  #!/usr/bin/env bash
  set -euo pipefail
  IFS=$'\t\n'
  gh issue list -L 2000 --json title --jq '.[] | [.title]' | sed -E 's/\["(.*)(\(P[0-9]\))?"]/\1/g' | sort | uniq --repeated
# --template '{{range .}}{{.number .title}}{{"\n"}}{{end}}
# --jq '.[] | [.number, .title]'

# Create an GH issue for each TODO line in the book's Markdown files
[unix]
create_issues_for_markdown:
  #!/usr/bin/env bash
  set -euo pipefail
  IFS=$'\n\t'
  for file in $(find {{root}}/src {{root}}/drafts -type f -name "*.md" -not -name "*refs.md")
  do
    base=$(basename $file '.md')
    rel=$(realpath --relative-to={{root}} $file)
    echo ">> $rel"
    for todo in $(sed -nE 's~^\s*TODO\s*:?\s*(.*)$~\1~pg' $file)
    do
      issue_url=$(gh issue create --title "${rel}: ${todo:-fix}" --body "[$rel](https://github.com/john-cd/rust_howto/blob/main/bk/${rel})" --label "markdown,auto" --assignee "@me")
      echo "$issue_url"
      # Escape [ and ]
      todo=$( echo "$todo" | sed -E 's/[[]/\\[/g; s/[]]/\\]/g' )
      sed -E -i 's~('^\s*TODO\s*:?\s*'"${todo}"')~[\1]('"${issue_url}"')~' $file
      sleep 3
    done
  done

# Create an GH issue for each TODO line in code examples
[unix]
create_issues_for_examples:
  #!/usr/bin/env bash
  set -euo pipefail
  IFS=$'\n\t'
  for file in $(find {{root}} -type f -name "*.rs" )
  do
    base=$(basename $file)
    rel=$(realpath --relative-to={{root}}/crates $file)
    echo ">> $rel"
    for todo in $(sed -nE 's~^\s*//\s*(//\s*)?TODO\s*:?\s*(.*)$~\2~pg' $file)
    do
      todo=$( echo "$todo" | sed -E 's/[[]/\\[/g; s/[]]/\\]/g' )
      echo ">${todo}<"
      issue_url=$(gh issue create --title "${rel}: ${todo:-fix}" --body "[$rel](https://github.com/john-cd/rust_howto/blob/main/bk/crates/${rel})" --label "code example,auto" --assignee "@me")
      echo "$issue_url"
      sed -E -i 's~TODO\s*:?\s*('"${todo}"')~[\1]('"${issue_url}"')~' $file
      sleep 3
    done
  done

# Create an GH issue for each TODO line in other files (*.sh, *.toml, *.yaml...)
[unix]
create_issues_for_other_files:
  #!/usr/bin/env bash
  set -euo pipefail
  IFS=$'\n\t'
  dir="{{root}}/../"
  for file in $(find "${dir}" -type f -name "*.sh" -o -name "*.just" -o -name "justfile" -o -name "*.toml" -o -name "*.yml" -o -name "*.yaml" -o -name "Dockerfile")
  do
    base=$(basename ${file})
    rel=$(realpath --relative-to=${dir} ${file})
    echo ">> ${rel}"
    for todo in $(sed -nE 's~^\s*(//\s*(//\s*)?|#+)\s*TODO\s*:?\s*(.*)$~\3~p' $file)
    do
      todo=$( echo "$todo" | sed -E 's/[[]/\\[/g; s/[]]/\\]/g' )
      echo ">${todo}<"
      issue_url=$(gh issue create --title "${rel}: ${todo:-fix}" --body "[$rel](https://github.com/john-cd/rust_howto/blob/main/${rel})" --label "auto" --assignee "@me")
      echo "$issue_url"
      sed -E -i 's~TODO\s*:?\s*('"${todo}"')~[\1]('"${issue_url}"')~' $file
      sleep 3
    done
  done
