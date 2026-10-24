#!/usr/bin/env bash
# TODO Reviewed ??-??-2026

set -euo pipefail

# WORK IN PROGRESS
# Create reference-style links and reference definitions, which can be used to replace bare URLs in the Markdown
# Manual review necessary
# Requires ripgrep
#
# Usage: ./scripts/urls/<script_name>.sh <root folder> [output path]
# Alternatively, set INPUT_PATH and optionally OUTPUT_PATH.

input_path="${1:-${INPUT_PATH:-}}"
output_path="${2:-${OUTPUT_PATH:-}}"

if [[ -z "${input_path}" ]]; then
  echo "Error: No input path provided." >&2
  echo "Usage: $0 <input_path> [output_path]" >&2
  exit 1
fi

if [[ ! -d "${input_path}" ]]; then
  echo "Error: Input path '${input_path}' does not exist or is not a directory." >&2
  exit 1
fi

root="$(cd -- "${input_path}" && pwd -P)/"

# Determine search paths: prefer src/ and drafts/ if they exist, otherwise use root.
search_paths=()
for dir in "src" "drafts"; do
  if [[ -d "${root}${dir}" ]]; then
    search_paths+=("${root}${dir}")
  fi
done

if [[ ${#search_paths[@]} -eq 0 ]]; then
  search_paths=("${root}")
fi

# [pass a var](https://github.com/john-cd/rust_howto/issues/1243)
patterns=(
  '(?<!: |["`([])(https?://(?:www\d?\.)?)(?!github\.com/)([^./?#\s]+)([A-Za-z0-9/._~?&=%#-]*)' '[`$2`][$2~website] [$2~website]: $1$2$3'
  '(?<!: |["`([])(https?://github\.com/)([A-Za-z0-9_.-]+)/([A-Za-z0-9_.-]+)' '[`$2/$3`][$2~$3~repo] [$2~$3~repo]: $1$2/$3'
)

markdown_without_code() {
  awk '
    {
      line = $0
      sub(/^[[:space:]]+/, "", line)
      marker = substr(line, 1, 3)
      if (!in_fence && (marker == "```" || marker == "~~~")) {
        fence_character = substr(line, 1, 1)
        fence_length = 0
        while (substr(line, fence_length + 1, 1) == fence_character) fence_length++
        in_fence = 1
        next
      }
      if (in_fence) {
        closes = length(line) >= fence_length
        for (i = 1; closes && i <= fence_length; i++) {
          if (substr(line, i, 1) != fence_character) closes = 0
        }
        if (closes && substr(line, fence_length + 1) !~ /[^[:space:]]/) in_fence = 0
        next
      }
      gsub(/`[^`]*`/, "", $0)
      print $0
    }
  ' "$1"
}

process_files() {
  while IFS= read -r -d '' file; do
    echo ">> $file" >&2
    contents=$(markdown_without_code "$file")
    # Look for http(s)://... and outputs references
    # Outputs reference-style links
    {
      for ((i=0; i<${#patterns[@]}; i+=2)); do
        pattern="${patterns[$i]}"
        replacement="${patterns[$i+1]}"
        printf '%s\n' "${contents}" | rg --pcre2 --only-matching --replace "${replacement}" "${pattern}" || true
      done
    # --pcre2 = Perl regex enabled (allows look-arounds) -g = glob, -r = replace
    } | sed -E -e 's=/$==' -e 's/[.,;!?]+$//' | sort -u
  done < <(find "${search_paths[@]}" -type f -name "*.md" -not -name "refs.incl.md" -not -name "SUMMARY.md" -not -name "*refs.md" -print0)
}

if [[ -n "${output_path}" ]]; then
  process_files > "${output_path}"
else
  process_files
fi

echo "DONE"
