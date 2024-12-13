#! /bin/bash
set -euo pipefail
IFS=$'\n\t'

# Quick and dirty script
# Outputs the contents of index of examples `src/examples_index.md`
# This script reads the local tables of content of all subchapters
#
# Usage:
# /code/scripts/index_of_examples/generate_index_of_examples.sh > src/examples_index.md

clean() {
    echo "$1" | sed -E '
    s/.*/\L&/;
    s/[a-z]*/\u&/g;
    s/\s(In|Of|And|With)\s/\L&/g;
    s/(Ansi|Uuid|Ffi|Os|Wasm|bsd|Gpu|Api|Gui|Lru|cv|Cd|Ci|Csv|Aws|Cors|Http|Ide|sql|ql|Tui)/\U&/g;
    s/Asref/`AsRef`/g;
    s/Cow/`Cow`/g;
    s/Grpc/gRPC/g;
    s/Mdbook/mdBook/g;
    s/(Tar|Cwd|Miri|Just|Rhai|Actix|Axum|Hyper)/\L`&`/g;
    s/\b(Option|Result)\b/`&`/g'
}

# Print the header
echo $'# Index of Examples\n'

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
    echo -e "## ${last}\n"
    # Iterate all subchapter TOCs, ignoring hidden ones
    for file in $(find $dir -type f -name '[^_]*.incl.md' -not -name "refs.incl.md")
    do
        incl=$(realpath --relative-to=${root}src $file)
        base=$(basename $file)
        # Titlecase and replace _ by space
        title=$(echo ${base%.incl.md} | sed 's/_/ /g')
        title=$(clean "$title")
        if [[ $title != "Index" ]]; then
          echo -e "### ${title}\n"
        fi
        echo -e "{{#include ${incl}}}\n"
    done
done

# Print the footer
cat << 'EOF'
{{#include refs.incl.md}}
{{#include refs/link-refs.md}}

<div class="hidden"></div>
EOF
