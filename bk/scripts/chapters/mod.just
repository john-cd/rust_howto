set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- CHAPTER MANAGEMENT -----------------------------------

# List chapters (pages) in Wikilink format
[unix]
list:
  #! /usr/bin/env bash
  files=$( find {{root}}/src -type f -name "*.md" -not -name "*.incl.md" -not -name "SUMMARY.md" -not -name "*refs.md" )
  for file in ${files}
  do
    base=$(echo "$file" | sed -E 's/[_]?index.md$//' | xargs basename -s ".md")
    clean=$(echo "${base}" | tr '_-' ' ' | sed -E 's/\w+/\u&/g; s/\b(api|uav|orm)(.?)\b/\U\1\E\2/gI; s/\b(aead|amqp|aws|cd|ci|cli|cow|csv|ffi|gtk|http|html|hmac|ini|json|lru|mime|miri|mssql|os|toml|tls|tui|ui|url|uuid|xml|yaml|gui|wasm|2d)\b/\U&/I; s/\b(in|of|and|for)\b/\L&/gI' )
    echo "[[${base} | ${clean}]]"
  done | sort -u

# Hides some markdown sections/ pages by adding a _ prefix to all filenames listed in `hiddenfiles.txt`
[confirm]
[unix]
@hide:
  ./hide.sh {{root}}

# Make all markdown sections / pages visible
[confirm]
[unix]
@unhide:
  ./unhide.sh {{root}}
