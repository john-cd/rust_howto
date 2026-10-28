#!/usr/bin/env bash
# Purpose: Convert {{#example <name>}} placeholders into ```rust {#include ...}``` blocks.
# Usage: ./scripts/examples/convert_example_placeholders.sh <root folder>.
#
# TODO Reviewed ??-??-2026.

set -eu

# Convert {{#example <name>}} placeholders into ```rust {#include ...}``` blocks
# and create the necessary code stubs (in subfolders of crates/**/examples/)
#

root="$(realpath $1)/"

for file in $(find ${root}src -type f -name "*.md" -not -name "*.incl.md" )
do
  # Grab the name of the example(s) to create and iterate
  # grep -P = Perl regex; -o = show only nonempty parts of lines that match; -h =  suppress the file name prefix on output
  # (?<= ) = lookbehind; (?= ) = lookahead
  # read -r = do not allow backslashes to escape any characters
  grep -Poh '(?<=\{\{#example ).+?(?=\}\})' $file | while read -r examplename ;
  do
    if [[ -n "${examplename}" ]]; then
        echo "Processing example: ${examplename}"
        current_file_dir=$(dirname -- "$file")
        source_dir=${current_file_dir#"$root"src/}
        folder_in_examples=$(basename -- "$file" .md)

        if [[ "$source_dir" == categories/* ]]; then
          category=${source_dir#categories/}
          category=${category%%/*}
          crate_path="cats/$(tr '-' '_' <<< "$category")"
        else
          section=${source_dir%%/*}
          if [[ -z "$section" ]]; then
            echo "Cannot determine the example crate for $file" >&2
            exit 1
          fi
          crate_path=$(tr '-' '_' <<< "$section")
        fi

      absoluteexampledir="${root}crates/${crate_path}/examples/${folder_in_examples}"
      exampledir=$(realpath --relative-to="$current_file_dir" "$absoluteexampledir" | tr '-' '_')
      # echo "exampledir: $exampledir"
      examplefilename=$(tr '-' '_' <<< ${examplename})
      examplefile="${exampledir}/${examplefilename}.rs"
      #echo "examplefile: $examplefile"
      sed -Ei 's=\{\{#example\s*?'${examplename}'\s*?\}\}=```rust,editable\n\{\{#include '$examplefile':example\}\}\n```=g' $file
      # Create the folder for the test, if missing
      mkdir -p $absoluteexampledir
      # Create a GitHub ticket
      issue=$( gh issue create --title "Add example for: ${examplename}" --body "[${examplefilename}](https://github.com/john-cd/rust_howto/tree/develop/$(realpath --relative-to=${root} ${absoluteexampledir})/${examplefilename}.rs )" )
      sleep 3
      # Add a stub file for the example
      cat > "$absoluteexampledir/${examplefilename}.rs" <<- EOF
// ANCHOR: example
// COMING SOON
// ANCHOR_END: example
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "not yet implemented"]
    fn test_main() {
        main();
    }
}
// [add example](${issue})
EOF

      # Add the example file as a module to `main.rs`
      if [[ -z $(grep -Foh "${examplefilename}" "$absoluteexampledir/main.rs") ]]; then
        echo " Adding to $absoluteexampledir/main.rs"
        cat >> "$absoluteexampledir/main.rs" <<- EOF
mod ${examplefilename};
EOF
      fi
    fi
  done
done

echo "DONE"
