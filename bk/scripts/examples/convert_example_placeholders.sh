#!/usr/bin/env bash
set -eu

# Convert {{#example <name>}} placeholders into ```rust {#include ...}``` blocks
# and create the necessary code stubs (in subfolders of crates/**/tests/)
#
# Usage: ./scripts/examples/convert_example_placeholders.sh <root folder>

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
        # TODO does not handle non-category examples
        current_file_dir=$(dirname $file)
        category=$(basename $current_file_dir | tr '-' '_')
        folder_in_tests=$(basename $file '.md')
    #     case ${category:0:1} in

    #     "a"|"b")
    #       example_crate_name="ab"
    #       ;;

    #     "c")
    #       example_crate_name="c"
    #       ;;

    #     "d")
    #       example_crate_name="d"
    #       ;;
    #     "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l")
    #       example_crate_name="efghijkl"
    #       ;;
    #     "m" | "n")
    #       example_crate_name="mn"
    #       ;;
    #     "o" | "p" | "q" |"r")
    #       example_crate_name="opqr"
    #       ;;
    #     "s" | "t" | "u" | "v")
    #       example_crate_name="stuv"
    #       ;;
    #     "w" | "x" | "y" | "z")
    #       example_crate_name="wxyz"
    #       ;;
    #     *)
    #       exit 108
    #       ;;
    #   esac

      absoluteexampledir="${root}crates/cats/${category}/tests/${folder_in_tests}"
      exampledir=$(realpath --relative-to=$current_file_dir "${absoluteexampledir}" | tr '-' '_')
      # echo "exampledir: $exampledir"
      examplefilename=$(tr '-' '_' <<< ${examplename})
      examplefile="${exampledir}/${examplefilename}.rs"
      #echo "examplefile: $examplefile"
      sed -Ei 's~\{\{#example\s*?'${examplename}'\s*?\}\}~```rust,editable\n\{\{#include '$examplefile':example\}\}\n```~g' $file
      # Create the folder for the test, if missing
      mkdir -p $absoluteexampledir
      # Create a GitHub ticket
      issue=$( gh issue create --title "Add example for: ${examplename}" --body "[${examplefilename}](https://github.com/john-cd/rust_howto/tree/main/$(realpath --relative-to=${root} ${absoluteexampledir})/${examplefilename}.rs )" )
      sleep 3
      # Add a stub file for the example
      cat > "$absoluteexampledir/${examplefilename}.rs" <<- EOF
// ANCHOR: example
// COMING SOON
// ANCHOR_END: example
fn main() {}

#[test]
#[ignore = "not yet implemented"]
fn test() {
    main();
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
