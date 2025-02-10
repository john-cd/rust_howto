set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := justfile_directory()

_default:
  @just --list gh --unsorted --justfile {{justfile()}}
#@just --choose

@login:
  gh auth login

# Create a GH issue with a title and an empty body
@issue title:
  gh issue create --title {{title}} --body ''

# Return a list of GitHub issues with the same title
find_dupe_issues:
  #! /bin/bash
  set -euo pipefail
  IFS=$'\t\n'
  gh issue list -L 2000 --json title --jq '.[] | [.title]' | sed -E 's/\["(.*)(\(P[0-9]\))?"]/\1/g' | sort | uniq --repeated
# --template '{{range .}}{{.number .title}}{{"\n"}}{{end}}
# --jq '.[] | [.number, .title]'

# Create an GH issue for each TODO line in Markdown files
create_issues_for_markdown:
  #! /bin/bash
  set -euo pipefail
  IFS=$'\n\t'
  for file in $(find {{root}}/src {{root}}/drafts -type f -name "*.md" -not -name "*refs.md")
  do
    base=$(basename $file '.md')
    rel=$(realpath --relative-to={{root}} $file)
    echo ">> $rel"
    ## extract TODO P0/1/2/3 ...
    for todo in $(grep -Eoh '\s*TODO\s*(P[0-9])?:?\s+.*$' $file)
    do
      #echo "gh issue create --title '${rel}: ${todo:-fix (P1)}' --body '[${rel}](https://github.com/john-cd/rust_howto/blob/main/${rel})'; sleep 3"
      issue_url=$(gh issue create --title "${rel}: ${todo:-fix (P1)}" --body "[$rel](https://github.com/john-cd/rust_howto/blob/main/${rel})" --label "markdown,auto")
      # Escape [ and ]
      echo "$issue_url"
      todo=$( echo "$todo" | sed -E 's/[[]/\\[/g; s/[]]/\\]/g' )
      sed -E -i 's~('${todo}')~[\1]('${issue_url}')~' $file
      sleep 3
    done
  done

# Create an GH issue for each TODO line in .rs files
create_issues_for_examples:
  #! /bin/bash
  set -euo pipefail
  IFS=$'\n\t'
  for file in $(find {{root}} -type f -name "*.rs" )
  do
    base=$(basename $file)
    rel=$(realpath --relative-to={{root}}/crates/ex $file)
    echo ">> $rel"
    for todo in $(sed -nE 's~^ *// *TODO(.*)$~\1~pg' $file)
    do
      todo=$( echo "$todo" | sed -E 's/[[]/\\[/g; s/[]]/\\]/g' )
      echo ">${todo}<"
      issue_url=$(gh issue create --title "${rel}: ${todo:-fix (P1)}" --body "[$rel](https://github.com/john-cd/rust_howto/blob/main//crates/ex/${rel})" --label "code example,auto")
      echo "$issue_url"
      # Escape [ and ]
      sed -E -i 's~TODO('${todo}')~[\1]('${issue_url}')~' $file
      sleep 3
    done
  done

## TODO

## Delete issues with --label "markdown,auto"
# delete_issues:
#   #! /bin/bash
#   issues=$(gh issue list --label "markdown,auto" --jq '.[].number' --json number)
#   for issue in $issues
#   do
#     gh issue delete $issue --yes
#     sleep 2
#   done

## TODO P1

## HACK: Replace TODO lines in the markdown by a link to previously created GitHub Issues
#add_gh_links:
#  #! /bin/bash
#  set -euo pipefail
#  IFS=$'\t\n'
#  for file in $(find {{root}} -type f -name "*.md")
#  do
#    base=$(basename $file '.md')
#    if [ $base == "_index" ] || [ $base == "index" ]; then
#     echo ">> ${file} SKIP"
#     continue
#    fi
#    rel=$(realpath --relative-to={{root}} $file )
#    echo ">> $rel"
#    ## extract strings after TODO P0/1/2/3...
#    for todo in $(sed -nE -e 's~^\s*TODO\s*(P[0-9])?:?\s*([^[]+?)$~\2~pg' $file )
#    do
#      echo "${base}: ${todo:-fix (P1)}"
#      ## Search for GitHub issue(s) that match the string
#      ## https://pkg.go.dev/text/template
#      link=$(gh issue list --search "${base}: ${todo:-fix (P1)}" --json title,url --template '{{{{range .}}[{{{{.title}}]({{{{.url}})\n{{{{end}}')
#      if [[ -n "$link" ]]; then
#        # Replace the TODO line by a [ issue title ]( issue url )
#        sed -E -i 's~^(\s*TODO\s*(P[0-9])?\s*'${todo}')$~'${link}'~g' $file
#      fi
#    done
#  done

## TODO P1

# HACK
add_gh_links_to_code:
  #! /bin/bash
  set -euo pipefail
  IFS=$'\t\n'
  for file in $(find {{root}} -type f -name "*.rs")
  do
    base=$(basename $file)
    rel=$(realpath --relative-to={{root}} $file )
    echo ">> $rel"
    ## Search for GitHub issue(s) that match the string
    ## https://pkg.go.dev/text/template
    link=$(gh issue list --search "$base" --json title,url --template '{{{{range .}}[{{{{.title}}]({{{{.url}})\n{{{{end}}')
    echo ">${link}<"
    if [[ -n "$link" ]]; then
        # Append [ issue title ]( issue url )
        sed -E -i '$ a // '${link} $file
    fi
  done

## Deduplicate .rs file (no sort - dup. lines must be consecutive)
#dedupe:
#  #! /bin/bash
#  set -euo pipefail
#  IFS=$'\t\n'
#  for file in $(find {{root}} -type f -name "*.rs")
#  do
#    echo $file
#    #sed -nr '$!N;/^(.*)\n\1$/!P;D' $file > /tmp/temp.rs
#    uniq $file > /tmp/temp.rs
#    mv -f /tmp/temp.rs $file
#  done

## TODO

## move // [...]... lines to the bottom of the file
#moveline:
#  #! /bin/bash
#  set -euo pipefail
#  IFS=$'\n\t'
#  for file in $(find {{root}} -type f -name "*.rs" )
#  do
#   cat <(grep -vE '\s*//\s*\[.*' $file) <(grep -E '\s*//\s*\[.*' $file) > /tmp/temp.rs
#   mv -f /tmp/temp.rs $file
#  done
