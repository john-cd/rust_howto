#! /bin/bash
set -euo pipefail
IFS=$'\n\t'

# Quick and dirty generation of the index of examples `examples_index.md`; manual editing required

clean() {
    echo "$1" | sed -E 's/.*/\L&/; s/[a-z]*/\u&/g; s/(In|Of|And|With)/\L&/g; s/(Ansi|Uuid|Ffi|Os|Wasm|bsd|Gpu|Api|Gui|Lru|cv|Cd|Ci|Csv|Aws|Cors|Http|Ide|ql|Tui)/\U&/g; s/-/ /g; s/Asref/AsRef/g; s/Grpc/gRPC/g'
}


clear
echo $'# Index of Examples\n'

# Leaf directories only
# https://stackoverflow.com/questions/4269798/use-gnu-find-to-show-only-the-leaf-directories
for dir in $(find ./src/* -type d -not -path "./src/refs" | sort -r | awk 'a!~"^"$0{a=$0;print}' | sort)
do
    # Ignore chapters that are hidden
    if [[ -f "$dir/_index.md" ]]; then
      continue
    fi
    # Last part of path, titlecased; FFI, OS... capitalized
    last=$(basename $dir | sed 's/_/: /g')
    last=$(clean $last)
    echo -e "## ${last}\n"
    # Iterate all subchapter TOCs, ignoring hidden ones
    for file in $(find $dir -type f -name '[^_]*.incl.md' -not -name "refs.incl.md")
    do
        incl=$(realpath --relative-to=./src $file)
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
echo << 'EOF'

{{#include refs.incl.md}}
{{#include refs/link-refs.md}}

<div class="hidden"></div>
EOF
