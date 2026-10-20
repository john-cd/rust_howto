#!/usr/bin/env bash
set -u

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

root="$(realpath "${input_path}")/"

if [[ ! -d "${root}" ]]; then
  echo "Error: Input path '${root}' does not exist or is not a directory." >&2
  exit 1
fi

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
  '(?<!: |["`([])(http(?:s)?://(?:www\d?\.)?)([^./]+)(\S+)?' '[`$2`][$2~website] [$2~website]: $1$2$3'
  '(?<!: |["`([])(http(?:s)?://(?:github\.com/)?)([^./]+)(\S+)?' '[`$2`][$2~repo] [$2~repo]: $1$2$3'
)

process_files() {
  for file in $( find "${search_paths[@]}" -type f -name "*.md"  -not -name "refs.incl.md" -not -name "SUMMARY.md" -not -name "*refs.md" )
  do
    echo ">> $file" >&2
    contents=$(rg --multiline --invert-match '`.*`' "$file")
    # Look for http(s)://... and outputs references
    # Outputs reference-style links
    {
      for ((i=0; i<${#patterns[@]}; i+=2)); do
        pattern="${patterns[$i]}"
        replacement="${patterns[$i+1]}"
        echo "${contents}" | rg --pcre2 --only-matching -r "${replacement}" "${pattern}"
      done
    # --pcre2 = Perl regex enabled (allows look-arounds) -g = glob, -r = replace
    } | sed 's=/$==' | sort
  done
}

if [[ -n "${output_path}" ]]; then
  process_files > "${output_path}"
else
  process_files
fi

echo "DONE"
