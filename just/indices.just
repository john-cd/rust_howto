_default:
  @just --list indices --unsorted --justfile {{justfile()}}
#@just --choose

## ---- INDICES MANAGEMENT -----------------------------------

# Outputs the contents of index of examples `src/examples_index.md` (reading the local tables of content of all subchapters)
generate_index_of_examples:
  {{justfile_directory()}}/scripts/index_of_examples/generate_index_of_examples.sh > src/examples_index.md
# Usage: just generate_index_of_examples

# Add, to `src/refs.incl.md`, missing references that are required for the index of examples (found in `examples_index.md`)
update_refdefs_for_index_of_examples:
  {{justfile_directory()}}/scripts/index_of_examples/update_refdefs_for_index_of_examples.sh
# Usage: just update_refdefs_for_index_of_examples

# Quick and dirty generation of language/index.md; manual editing required
generate_language_index:
  {{justfile_directory()}}/scripts/language/generate_language_index.sh
# Usage: generate_language_index >> src/language/index.incl.md

# Generate the summary references for the dev tool category
generate_dev_tools_refdefs:
  #! /bin/bash
  root="{{justfile_directory()}}/src/categories/development-tools/"
  mv -f ${root}refs.incl.md ${root}refs.incl.md.bak
  for file in $(find ${root} -mindepth 2 -type f -name "refs.incl.md")
  do
    rel=$(realpath --relative-to=${root} $file)
    rel=$(dirname $rel)'/'
    sed -E 's~^(\[.*\]:\s*?)(.*)$~\1'${rel}'\2~g' $file >> ${root}refs.incl.md
  done
  sort -u -o ${root}refs.incl.md ${root}refs.incl.md
