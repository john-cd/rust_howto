#! /bin/bash
set -eu

# Convert {{#example <name>}} placeholders into ```rust {#include ...}``` blocks
# and create the necessary code stubs (in subfolders of crates/**/tests/)
#
# Usage: /code/scripts/examples/convert_example_placeholders.sh

root="/code/"

for file in $(find ${root}src -type f -name "*.md" -not -name "*.incl.md" )
do
  # Grab the name of the example(s) to create and iterate
  # grep -P = Perl regex; -o = show only nonempty parts of lines that match; -h =  suppress the file name prefix on output
  # (?<= ) = lookbehind; (?= ) = lookahead
  # read -r = do not allow backslashes to escape any characters
  grep -Poh '(?<=\{\{#example ).+?(?=\}\})' $file | tr '-' '_' | while read -r examplename ;
  do
    if [[ -n $examplename ]]; then
        echo "Processing example: $examplename"
        # P2 does not handle non-category examples
        current_file_dir=$(dirname $file)
        category=$( basename $current_file_dir | tr '-' '_')
        # echo "rel: $rel"
        case ${category:0:1} in

        "a"|"b")
          example_crate_name="ab"
          ;;

        "c")
          example_crate_name="c"
          ;;

        "d")
          example_crate_name="d"
          ;;
        "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l")
          example_crate_name="efghijkl"
          ;;
        "m" | "n")
          example_crate_name="mn"
          ;;
        "o" | "p" | "q" |"r")
          example_crate_name="opqr"
          ;;
        "s" | "t" | "u" | "v")
          example_crate_name="stuv"
          ;;
        "w" | "x" | "y" | "z")
          example_crate_name="wxyz"
          ;;
        *)
          exit 108
          ;;
      esac

      absoluteexampledir="${root}crates/ex/categories/${example_crate_name}/tests/${category}"
      current_file_dir=$(dirname $file)
      exampledir=$(realpath --relative-to=$current_file_dir "${absoluteexampledir}" | tr '-' '_')
      # echo "exampledir: $exampledir"
      examplefile="${exampledir}/${examplename}.rs"
      #echo "examplefile: $examplefile"
      sed -Ei 's~\{\{#example\s*?'${examplename}'\s*?\}\}~```rust,editable\n\{\{#include '$examplefile':example\}\}\n```~g' $file
      # Create the folder for the test, if missing
      mkdir -p $absoluteexampledir
      # Add a stub file for the example
      issue=$( gh issue create --title "Add example ${examplename}" --body "${examplefile}" )
      cat > "$absoluteexampledir/${examplename}.rs" <<- EOF
// ANCHOR: example
fn main() {
    todo!();
    }
// ANCHOR_END: example

#[test]
#[ignore = "not yet implemented"]
fn test() {
    main();
}
// [P1 add example](${issue})
EOF

      # Add the example file as a module to `mod.rs`
      if [[ -z $(grep -Foh "${examplename}" "$absoluteexampledir/mod.rs") ]]; then
        echo " Adding to $absoluteexampledir/mod.rs"
        cat >> "$absoluteexampledir/mod.rs" <<- EOF
mod ${examplename};
EOF
      fi
    fi
  done
done

echo "DONE"
