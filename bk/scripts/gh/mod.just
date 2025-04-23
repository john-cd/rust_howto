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
  dir="{{root}}/../"
  prefix='^\s*TODO\s*:?\s*'
  for file in $(find {{root}}/src {{root}}/drafts -type f -name "*.md" -not -name "*refs.md")
  do
    base=$(basename $file '.md')
    rel=$(realpath --relative-to=${dir} $file)
    echo ">> $rel"
    for todo in $(sed -nE 's~'"${prefix}"'(.+)$~\1~pg' $file)
    do
      issue_url=$(gh issue create --title "${rel}: ${todo:-fix}" --body "[$rel](https://github.com/john-cd/rust_howto/blob/main/${rel})" --label "markdown,auto" --assignee "@me")
      echo "$issue_url"
      # Escape [ ] ? ( )
      todo=$( echo "$todo" | sed -E 's~[[]~\\[~g; s~[]]~\\]~g; s~([?()])~\\\1~g' )
      sed -E -i 's~'"${prefix}${todo}"'~['"${todo}"']('"${issue_url}"')~' $file
      sleep 3
    done
  done

# Create an GH issue for each TODO line in .rs files (incl. code examples)
[unix]
create_issues_for_code:
  #!/usr/bin/env bash
  set -euo pipefail
  IFS=$'\n\t'
  dir="{{root}}/../"
  prefix='^\s*//\s*(//\s*)?'
  pattern='TODO\s*:?\s*'
  for file in $(find ${dir} -type f -name "*.rs" )
  do
    base=$(basename $file)
    rel=$(realpath --relative-to=${dir} $file)
    echo ">> $rel"
    for todo in $(sed -nE 's~'"${prefix}${pattern}"'(.*)$~\2~pg' $file)
    do
      todo=$( echo "$todo" | sed -E 's~[[]~\\[~g; s~[]]~\\]~g; s~([?()])~\\\1~g' )
      echo ">${todo}<"
      issue_url=$(gh issue create --title "${rel}: ${todo:-fix}" --body "[$rel](https://github.com/john-cd/rust_howto/blob/main/${rel})" --label "code example,auto" --assignee "@me")
      echo "$issue_url"
      sed -E -i 's~'"${pattern}${todo}"'~['"${todo}"']('"${issue_url}"')~' $file
      sleep 3
    done
  done

# Create an GH issue for each TODO line in other files (*.sh, *.toml, *.yaml...)
[unix]
create_issues_for_other:
  #!/usr/bin/env bash
  set -euo pipefail
  IFS=$'\n\t'
  dir="{{root}}/../"
  prefix='^\s*(//\s*(//)?|#+)\s*'
  pattern='TODO\s*:?\s*'
  for file in $(find "${dir}" -type f -name "*.sh" -o -name "*.just" -o -name "justfile" -o -name "*.toml" -o -name "*.yml" -o -name "*.yaml" -o -name "Dockerfile")
  do
    base=$(basename ${file})
    rel=$(realpath --relative-to=${dir} ${file})
    echo ">> ${rel}"
    for todo in $(sed -nE 's~'"${prefix}${pattern}"'(.*)$~\3~p' $file)
    do
      todo=$( echo "$todo" | sed -E 's~[[]~\\[~g; s~[]]~\\]~g; s~([?()])~\\\1~g' )
      echo ">${todo}<"
      issue_url=$(gh issue create --title "${rel}: ${todo:-fix}" --body "[$rel](https://github.com/john-cd/rust_howto/blob/main/${rel})" --label "auto" --assignee "@me")
      echo "$issue_url"
      sed -E -i 's~'"${pattern}${todo}"'~['"${todo}"']('"${issue_url}"')~' $file
      sleep 3
    done
  done

# Refresh GitHub issues
[confirm]
[unix]
refresh:
  #!/usr/bin/env bash
  set -euo pipefail
  IFS=$'\n\t'
  dir="{{root}}/../"
  for file in $(find "${dir}" -type f -name "*.rs" -o -name "*.md" -o -name "*.toml" -o -name "*.yaml" -o -name "*.yml" -not -path '*/target/*' -not -path '*/bin/*' )
  do
    rel=$(realpath --relative-to=${dir} ${file})
    #echo "> ${rel}"
    for str in $( sed -En 's~^\s*\[([^]]*?)\]\(https://github.com/john-cd/rust_howto/issues/([0-9]+)\)~\1@\2@~p' "${file}" )
    do
      todo=$( echo "$str" | cut -d@ -f1 )
      id=$( echo "$str" | cut -d@ -f2 )
      echo "---"
      echo "id: ${id}"
      echo "title: ${rel}: ${todo:-fix}"
      echo "body: [$rel](https://github.com/john-cd/rust_howto/blob/main/${rel})"
      gh issue edit "${id}" --title "${rel}: ${todo:-fix}" --body "[$rel](https://github.com/john-cd/rust_howto/blob/main/${rel})" --add-assignee "@me" --add-label "auto"
      sleep 3
      break
    done
  done
