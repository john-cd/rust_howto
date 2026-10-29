#!/usr/bin/env bash
# Purpose: Inventory TODO notes in tracked files and optionally create one GitHub issue per note.
# Usage: todo_issues.sh [--apply] [--kind md|code|other|all] <repo root>
#
#   Default is a dry run: it lists the notes and the issues that would be
#   created, and makes no GitHub or file changes. With --apply, each note gets
#   an issue and its source line is rewritten to link to it.
#
# Scope: files tracked by git under the repo root only. Other worktrees,
# build links, untracked files, submodules, fixtures and templates are never
# scanned. `TODO Reviewed` markers are reserved for human review and are skipped.
#
# Environment: GH overrides the gh executable (used by test_todo_issues.sh).
#
# TODO Reviewed ??-??-2026.

set -euo pipefail

apply=0
kind=all
while [[ $# -gt 0 ]]; do
  case $1 in
    --apply) apply=1 ;;
    --kind)
      kind=${2:-}
      shift
      ;;
    --) shift; break ;;
    -*)
      echo "Unknown option: $1" >&2
      exit 2
      ;;
    *) break ;;
  esac
  shift
done
if [[ $# -ne 1 || ! -d $1 ]] || ! [[ $kind =~ ^(md|code|other|all)$ ]]; then
  echo "Usage: $0 [--apply] [--kind md|code|other|all] <repo root>" >&2
  exit 2
fi

gh_bin="${GH:-gh}"
root="$(cd -- "$1" && pwd -P)"
repo_url="https://github.com/john-cd/rust_howto/blob/develop"
# The marker, optionally preceded by a comment introducer, then the note text.
note_re='^([[:space:]]*(//[/!]?|#+|<!--)?[[:space:]]*)TODO[[:space:]]*:?[[:space:]]*(.*)$'
reviewed_re='^[[:space:]]*(//[/!]?|#+|<!--)?[[:space:]]*TODO[[:space:]]+Reviewed'

# Print the tracked files of the requested kind, NUL separated, without submodules, fixtures and templates.
list_files() {
  git -C "$root" ls-files -z --stage | while IFS= read -r -d '' entry; do
    mode=${entry%% *}
    path=${entry#*$'\t'}
    [[ $mode == 160000 ]] && continue
    case $path in
      .worktrees/* | testdata/* | */testdata/* | fixtures/* | */fixtures/* | templates/* | */templates/* | target/* | */target/* | build/* | */build/*) continue ;;
      *refs.md) continue ;;
    esac
    case $kind:$path in
      md:*.md | all:*.md) ;;
      code:*.rs | all:*.rs) ;;
      other:*.sh | other:*.just | other:justfile | other:*.toml | other:*.yml | other:*.yaml | other:Dockerfile) ;;
      all:*.sh | all:*.just | all:justfile | all:*.toml | all:*.yml | all:*.yaml | all:Dockerfile) ;;
      *) continue ;;
    esac
    printf '%s\0' "$path"
  done
}

# Existing issue titles (open and closed), used to avoid duplicates.
existing_titles=""
if [[ $apply -eq 1 ]]; then
  existing_titles="$("$gh_bin" issue list --state all -L 5000 --json title --jq '.[].title')"
fi

cr=$'\015'
created=0
skipped=0
while IFS= read -r -d '' rel; do
  file="${root}/${rel}"
  [[ -f $file ]] || continue
  lineno=0
  # shellcheck disable=SC2094  # the loop only reads $file; edits go through a temporary file.
  while IFS= read -r line || [[ -n $line ]]; do
    lineno=$((lineno + 1))
    eol=""
    if [[ $line == *"$cr" ]]; then
      line=${line%"$cr"}
      eol=$cr
    fi
    [[ $line =~ $reviewed_re ]] && continue
    [[ $line =~ $note_re ]] || continue
    prefix=${BASH_REMATCH[1]}
    text=${BASH_REMATCH[3]}
    title="${rel}: ${text:-fix}"
    if [[ $apply -eq 0 ]]; then
      printf 'would create: %s (line %s)\n' "$title" "$lineno"
      continue
    fi
    if grep -Fxq -- "$title" <<<"$existing_titles"; then
      echo "skip (issue exists): $title"
      skipped=$((skipped + 1))
      continue
    fi
    body="[${rel}](${repo_url}/${rel})"
    url="$("$gh_bin" issue create --title "$title" --body "$body" --label auto --assignee "@me")"
    # Only edit the source after the issue exists; write through a temporary file.
    tmp="$(mktemp "${file}.XXXXXX")"
    replacement="${prefix}[${text:-fix}](${url})${eol}"
    if REPLACEMENT="$replacement" awk -v n="$lineno" 'NR == n { print ENVIRON["REPLACEMENT"]; next } { print }' "$file" >"$tmp"; then
      mv -- "$tmp" "$file"
    else
      rm -f -- "$tmp"
      echo "Could not update ${rel}:${lineno}; issue was created: ${url}" >&2
      exit 1
    fi
    existing_titles+=$'\n'"$title"
    created=$((created + 1))
    echo "created: $url"
  done <"$file"
done < <(list_files)

if [[ $apply -eq 1 ]]; then
  echo "created ${created}, skipped ${skipped}"
else
  echo "dry run only: pass --apply to create issues"
fi
