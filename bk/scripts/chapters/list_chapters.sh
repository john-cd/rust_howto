#! /usr/bin/env bash
set -eu

root="$(realpath $1)"
keyword="${2:-""}"

if [ "$keyword" == "" ]; then
  files=$( find ${root}/src -type f -name "*.md" -not -name "*.incl.md" -not -name "SUMMARY.md" -not -name "*refs.md" )
else
  files=$( rg -F -i --files-with-matches -g "*.md" -g "!*.incl.md" -g "!SUMMARY.md" -g "!*refs.md" "${keyword}" ${root}/src )
fi

for file in ${files}
do
    base=$(echo "$file" | sed -E 's/[_]?index.md$//' | xargs basename -s ".md")
    clean=$(echo "${base}" | tr '_-' ' ' | sed -E 's/\w+/\u&/g; s/\b(api|uav|orm)(.?)\b/\U\1\E\2/gI; s/\b(aead|amqp|aws|cd|ci|cli|cow|csv|ffi|gtk|http|html|hmac|ini|json|lru|mime|miri|mssql|os|toml|tls|tui|ui|url|uuid|xml|yaml|gui|wasm|2d)\b/\U&/I; s/\b(in|of|and|for)\b/\L&/gI' )
    printf "%-50s %s\n" "[[${base} | ${clean}]]" "$(realpath --relative-to=${root}/src ${file})"
done | sort -u
