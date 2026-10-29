#! /usr/bin/env bash
# Purpose: List chapters containing wiki-style links, optionally filtered by keyword.
# Usage: Run from bk/ with just wikilinks list or just wikilinks for <keyword>.
#
# TODO Reviewed ??-??-2026.

set -eu

root="$(realpath "$1")"
root="${root//\\//}"
if [[ "$root" =~ ^/([a-zA-Z])/(.*)$ ]]; then
  root="${BASH_REMATCH[1]^^}:/${BASH_REMATCH[2]}"
fi
keyword="${2:-""}"

format_chapters() {
  while IFS= read -r -d '' file; do
    file="${file//\\//}"
    filename="${file##*/}"
    case "$filename" in
      index.md) parent="${file%/index.md}"; base="${parent##*/}" ;;
      *_index.md) base="${filename%_index.md}" ;;
      *.md) base="${filename%.md}" ;;
    esac

    case "$file" in
      "${root}/src/"*) relative="${file#"${root}/src/"}" ;;
      "${root}/drafts/"*) relative="../drafts/${file#"${root}/drafts/"}" ;;
      "${root}/../later/src/"*) relative="../../later/src/${file#"${root}/../later/src/"}" ;;
      *) printf 'Unexpected chapter path: %s\n' "$file" >&2; return 1 ;;
    esac

    printf '%s\t%s\n' "$base" "$relative"
  done | awk -F '\t' '
    BEGIN {
      split("aead amqp api aws cd ci cli cow csv ffi gui gtk hmac html http ini json lru mime miri mssql orm os tls toml tui uav ui url uuid wasm xml yaml 2d", words, " ")
      for (i in words) uppercase[words[i]] = 1
      lowercase["in"] = lowercase["of"] = lowercase["and"] = lowercase["for"] = 1
    }
    function title(value, parts, count, i, word, result) {
      gsub(/[_-]/, " ", value)
      count = split(value, parts, / +/)
      for (i = 1; i <= count; i++) {
        word = tolower(parts[i])
        if (word in uppercase) word = toupper(word)
        else word = toupper(substr(word, 1, 1)) substr(word, 2)
        if (tolower(parts[i]) in lowercase) word = tolower(word)
        result = result (i == 1 ? "" : " ") word
      }
      return result
    }
    { printf "%-50s %s\n", "[[" $1 " | " title($1) "]]", $2 }
  '
}

if [ "$keyword" == "" ]; then
  find "${root}/src" "${root}/drafts" "${root}/../later/src" -type f -name "*.md" -not -name "*.incl.md" -not -name "SUMMARY.md" -not -name "*refs.md" -print0 |
    format_chapters | sort -u
else
  rg --null -F -i --files-with-matches -g "*.md" -g "!*.incl.md" -g "!SUMMARY.md" -g "!*refs.md" "${keyword}" "${root}/src" "${root}/drafts" "${root}/../later/src" |
    format_chapters | sort -u
fi
