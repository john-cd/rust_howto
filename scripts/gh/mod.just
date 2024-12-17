set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
  @just --list gh --unsorted --justfile {{justfile()}}
#@just --choose

#login:
#  gh auth login

# Create a GH issue with a title and an empty body
issue title:
  gh issue create --title {{title}} --body ''

# Return a list of GitHub issues with the same title
find_dupe_issues:
  #! /bin/bash
  set -euo pipefail
  IFS=$'\t\n'
  gh issue list -L 1000 --json title --jq '.[] | [.title]' | sed -E 's/\["(.*)(\(P[0-9]\))?"]/\1/g' | sort | uniq --repeated
# --template '{{range .}}{{.number .title}}{{"\n"}}{{end}}
# --jq '.[] | [.number, .title]'

## Create an GH issue for each TODO line in Markdown files
#create_issues_for_markdown:
#  #! /bin/bash
#  set -euo pipefail
#  IFS=$'\n\t'
#  for file in $(find {{justfile_directory()}}/src {{justfile_directory()}}/drafts -type f -name "*.md")
#  do
#    echo ">>$file"
#    # extract TODO P0/1/2/3 ...
#    todos=$(sed -nE 's~^#*\s*TODO\s*(P[0-9])?\s*(.*)$~\2 (\1)~pg' $file)
#    for todo in $todos
#    do
#      base=$(basename $file '.md')
#      path=$(realpath --relative-to={{justfile_directory()}} $file)
#      echo "gh issue create --title '${base}: ${todo:-fix (P1)}' --body '[${path}](https://github.com/john-cd/rust_howto/blob/main/${path})'; sleep 3"
#      ## Or call the Github CLI directly
#      #gh issue create --title "${base}: ${todo:-fix (P1)}" --body "[$path](https://github.com/john-cd/rust_howto/blob/main/${path})" \
#      #  --label "markdown,auto"
#      #sleep 3
#    done
#  done
#  # Also consider checking against existing issues e.g.
#  # existing=issues=$(gh issue list --label "markdown,auto" --jq '.[].title' --json number,title)

## Create an GH issue for each TODO line in Rust code (.rs) files
# create_issues_for_examples:
#   #! /bin/bash
#   set -euo pipefail
#   IFS=$'\n\t'
#   for file in $(find {{justfile_directory()}} -type f -name "*.rs" )
#   do
#     todos=$(sed -nE 's~^//\s*TODO\s*(P[0-9])?\s*(.*)$~\2 (\1)~pg' $file)
#     for todo in $todos
#     do
#       base=$(basename $file '.rs')
#       echo "${base}: ${todo}"
#       gh issue create --title "${base}: ${todo}" --body "$file" --label "code example" --label "auto"
#       echo "--------------------------"
#       sleep 2
#     done
#   done

# delete_issues:
#   #! /bin/bash
#   issues=$(gh issue list --label "markdown,auto" --jq '.[].number' --json number)
#   for issue in $issues
#   do
#     gh issue delete $issue --yes
#     sleep 2
#   done
