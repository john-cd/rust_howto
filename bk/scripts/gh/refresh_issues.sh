#!/usr/bin/env bash
# Purpose: Refresh the title and source link of GitHub issues referenced from tracked source files.
# Usage: refresh_issues.sh [--apply] <repo root>
#
#   Default is a read-only preview. With --apply, an open issue whose title
#   differs gets the generated title, and its source link is written into a
#   delimited generated section of the body:
#
#     <!-- source-link:start --> ... <!-- source-link:end -->
#
#   Everything outside that section (acceptance criteria, blocker history,
#   discussion) is preserved byte for byte. Assignees and labels are never changed.
#   Closed issues are skipped.
#
# Scope: files tracked by git under the repo root only; other worktrees,
# submodules, fixtures, templates and build output are excluded.
#
# Environment: GH overrides the gh executable (used by test_refresh_issues.sh).
#
# TODO Reviewed ??-??-2026.

set -euo pipefail

apply=0
if [[ ${1:-} == "--apply" ]]; then
  apply=1
  shift
fi
if [[ $# -ne 1 || ! -d $1 ]]; then
  echo "Usage: $0 [--apply] <repo root>" >&2
  exit 2
fi

gh_bin="${GH:-gh}"
root="$(cd -- "$1" && pwd -P)"
repo_url="https://github.com/john-cd/rust_howto/blob/develop"
start_marker='<!-- source-link:start -->'
end_marker='<!-- source-link:end -->'
# A line that is just a Markdown link to an issue of this repository, optionally after a comment introducer.
link_re='^[[:space:]]*(//[/!]?|#+|<!--)?[[:space:]]*\[([^]]*)\]\(https://github.com/john-cd/rust_howto/issues/([0-9]+)\)'

# Print the tracked files to scan, NUL separated.
list_files() {
  git -C "$root" ls-files -z --stage | while IFS= read -r -d '' entry; do
    mode=${entry%% *}
    path=${entry#*$'\t'}
    [[ $mode == 160000 ]] && continue
    case $path in
      .worktrees/* | testdata/* | */testdata/* | fixtures/* | */fixtures/* | templates/* | */templates/* | target/* | */target/* | build/* | */build/*) continue ;;
      *.rs | *.md | *.toml | *.yml | *.yaml | *.sh | *.just | justfile | Dockerfile) printf '%s\0' "$path" ;;
    esac
  done
}

# Return the body with the generated source-link section set to $2 (appended when absent).
with_section() {
  local body=$1 section=$2
  if [[ $body == *"$start_marker"*"$end_marker"* ]]; then
    local before=${body%%"$start_marker"*}
    local after=${body#*"$end_marker"}
    printf '%s%s\n%s\n%s%s' "$before" "$start_marker" "$section" "$end_marker" "$after"
  else
    printf '%s\n\n%s\n%s\n%s\n' "$body" "$start_marker" "$section" "$end_marker"
  fi
}

changed=0
while IFS= read -r -d '' rel; do
  file="${root}/${rel}"
  [[ -f $file ]] || continue
  while IFS= read -r line || [[ -n $line ]]; do
    line=${line%$'\r'}
    [[ $line =~ $link_re ]] || continue
    text=${BASH_REMATCH[2]}
    id=${BASH_REMATCH[3]}
    state="$("$gh_bin" issue view "$id" --json state --jq .state)"
    if [[ $state != OPEN ]]; then
      echo "skip #${id} (${state}): ${rel}"
      continue
    fi
    title="${rel}: ${text:-fix}"
    current_title="$("$gh_bin" issue view "$id" --json title --jq .title)"
    body="$("$gh_bin" issue view "$id" --json body --jq .body)"
    section="[${rel}](${repo_url}/${rel})"
    new_body="$(with_section "$body" "$section")"
    if [[ $current_title == "$title" && $new_body == "$body" ]]; then
      continue
    fi
    changed=$((changed + 1))
    if [[ $apply -eq 0 ]]; then
      printf 'would update #%s\n  title: %s -> %s\n  body: keep text, set generated section to %s\n' "$id" "$current_title" "$title" "$section"
      continue
    fi
    "$gh_bin" issue edit "$id" --title "$title" --body "$new_body"
    echo "updated #${id}"
  done <"$file"
done < <(list_files)

if [[ $apply -eq 0 ]]; then
  echo "preview only (${changed} change(s)): pass --apply to edit issues"
else
  echo "updated ${changed} issue(s)"
fi
