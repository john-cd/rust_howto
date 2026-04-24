#!/usr/bin/env bash
set -u

# Convert wiki-style links [[...|...]] to [...][p~...] and [p~...]: ...
# BEWARE: modifies many files.
#
# Usage <script>.sh <root folder of book>

# [handle [[...]] without title](https://github.com/john-cd/rust_howto/issues/1244)

root="$(realpath $1)/"

files=$(find ${root}src -type f \( -name "*.md" -not -name "SUMMARY.md" -not -name "examples_index.md" -not -name "*.incl.md" \))

# [[...]] or [[...|...]]
regex='\[\[([^|[:space:]\]]+)\s*(?:\|\s*([^]]+))?\]\]'

for file in ${files}
do
  echo -e ">> ${file}"

  dir=$(dirname ${file})
  for target_file_name in $( rg --no-line-number --no-filename --only-matching -r '$1' "${regex}" "${file}" )
  do
    #echo ">$target_file_name<"
    ## Retrieve the path in the list of files
    target_path=$(rg -o "\S+/${target_file_name}\.md|\S+${target_file_name}/[_]?index.md" <<< "${files}" )
    #echo "$target_path"
    ## Get filename or directory name if [_]?index.md
    base=$(basename -s ".md" "${target_path%/*index\.md}")
    rel_path=$(realpath --relative-to=${dir} ${target_path})
    echo "[p~${base}]: ${rel_path}" #>> "${dir}/refs.incl.md"
  done

  ## Replace [[...|...]] by [...][p~...] and [[...]] by [...][p~...]
  sed -E -i -e 's=\[\[([^]|[:space:]]+)\s*\|\s*([^]]+)\]\]=[\2][p~\1]=g' -e 's=\[\[([^]|[:space:]]+)\]\]=[\1][p~\1]=g' "${file}" # -n p

done
