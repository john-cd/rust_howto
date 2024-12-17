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
create_issues_for_markdown:
  #! /bin/bash
  set -euo pipefail
  IFS=$'\n\t'
  for file in $(find {{justfile_directory()}}/src {{justfile_directory()}}/drafts -type f -name "*.md" -not -name "*refs.md")
  do
    base=$(basename $file '.md')
    rel=$(realpath --relative-to={{justfile_directory()}} $file)
    #echo ">> $rel"
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
# TODO: add the GitHub url back to the .rs file

## Delete issues with --label "markdown,auto"
# delete_issues:
#   #! /bin/bash
#   issues=$(gh issue list --label "markdown,auto" --jq '.[].number' --json number)
#   for issue in $issues
#   do
#     gh issue delete $issue --yes
#     sleep 2
#   done

## HACK: Replace TODO lines in the markdown by a link to previously created GitHub Issues
#add_gh_links:
#  #! /bin/bash
#  set -euo pipefail
#  IFS=$'\t\n'
#  for file in $(find {{justfile_directory()}} -type f -name "*.md")
#  do
#    base=$(basename $file '.md')
#    if [ $base == "_index" ] || [ $base == "index" ]; then
#     echo ">> ${file} SKIP"
#     continue
#    fi
#    rel=$(realpath --relative-to={{justfile_directory()}} $file )
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

# HACK
add_gh_links_to_code:
  #! /bin/bash
  set -euo pipefail
  IFS=$'\t\n'
  for file in $(find {{justfile_directory()}} -type f -name "*.rs")
  do
    base=$(basename $file)
    rel=$(realpath --relative-to={{justfile_directory()}} $file )
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

#dedupe:
#  #! /bin/bash
#  set -euo pipefail
#  IFS=$'\t\n'
#  for file in $(find {{justfile_directory()}} -type f -name "*.rs")
#  do
#    echo $file
#    #sed -nr '$!N;/^(.*)\n\1$/!P;D' $file > /tmp/temp.rs
#    uniq $file > /tmp/temp.rs
#    mv -f /tmp/temp.rs $file
#  done
