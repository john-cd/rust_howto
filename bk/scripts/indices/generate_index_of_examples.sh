#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

# Quick and dirty script
# Outputs the contents of index of examples `src/examples_index.md`
# This script reads the local tables of content of all subchapters
#
# Usage:
# ./scripts/index_of_examples/generate_index_of_examples.sh <root folder>

clean() {
    echo "$1" | sed -E '
    s/.*/\L&/;
    s/[a-z]*/\u&/g;
    s/\s(In|Of|And|With)\s/\L&/g;
    s/(Ansi|Uuid|Ffi|Os|Wasm|bsd|Gpu|Api|Gui|Lru|cv|Cd|Ci|Csv|Aws|Cors|Http|Ide|sql|ql|Tui|Mssql|Amqp|Kv|Tls|Aead|Orm)/\U&/g;
    s/Asref/`AsRef`/g;
    s/Cow/`Cow`/g;
    s/Grpc/gRPC/g;
    s/Mdbook/mdBook/g;
    s/(Tar|Cwd|Miri|Just|Rhai|Actix|Axum|Hyper)/\L`&`/g;
    s/\b(Option|Result)\b/`&`/g'
}

root="$(realpath $1)/"

index_file="${root}src/examples_index.md"

hiddendiv=$( sed -n '/^<div class="hidden">/,/^<\/div>/ p' "${index_file}" )

# Print the header
echo $'# Index of Examples\n' > "${index_file}"


# Leaf directories only
# https://stackoverflow.com/questions/4269798/use-gnu-find-to-show-only-the-leaf-directories
for dir in $(find ${root}src/* -type d -not -path "${root}src/refs" -exec sh -c '(ls -p "{}"|grep />/dev/null)||echo "{}"' \;)
do
    # Ignore chapters that are hidden
    if [[ -f "$dir/_index.md" ]]; then
      continue
    fi
    # Get last part of path, titlecased; abbreviations capitalized
    # The folders in `src/categories` are based on `crates.io` slugs: replace - by ' ';  _ separates parent and child categories
    last=$(basename $dir | sed 's/-/ /g; s/_/: /g')
    last=$(clean $last)
    echo -e "## ${last}\n" >> "${index_file}"
    # Iterate all subchapter TOCs, ignoring hidden ones
    for file in $(find $dir -type f -name '[^_]*.incl.md' -not -name "refs.incl.md")
    do
        incl=$(realpath --relative-to=${root}src $file)
        base=$(basename $file)
        # Titlecase and replace _ by space
        title=$(echo ${base%.incl.md} | sed 's/_/ /g')
        title=$(clean "$title")
        if [[ $title != "Index" ]]; then
          echo -e "### ${title}\n" >> "${index_file}"
        fi
        echo -e "{{#include ${incl}}}\n" >> "${index_file}"
    done
done

# Print the footer
cat >> "${index_file}" << 'EOF'
{{#include refs.incl.md}}
{{#include refs/link-refs.md}}

EOF

echo "$hiddendiv" >> "${index_file}"
