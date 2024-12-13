#! /bin/bash
set -eu

# Convert {{#example <name>}} placeholders into ```rust {#include ...}``` blocks
# and create the necessary code stubs (in subfolders of deps/tests/)
#
# Usage: /code/scripts/examples/convert_example_placeholders.sh

root="/code/"

for file in $(find ${root}src -type f -name "*.md" -not -name "*.incl.md" )
do
  # Grab the name of the example(s) to create and iterate
  # grep -P = Perl regex; -o = show only nonempty parts of lines that match; -h =  suppress the file name prefix on output
  # read -r = do not allow backslashes to escape any characters
  grep -Poh '(?<=\{\{#example ).+?(?=\}\})' $file | tr '-' '_' | while read -r examplename ;
  do
    if [[ -n $examplename ]]; then
        echo "Processing example: $examplename"
        dir=$(dirname $file)
        rel=$(realpath --relative-to=${root}src $dir | tr '-' '_')
        # echo "rel: $rel"
        exampledir=$(realpath --relative-to=$dir "${root}deps/tests" | tr '-' '_')"/$rel"
        # echo "exampledir: $exampledir"
        examplefile="${exampledir}/${examplename}.rs"
        # echo "examplefile: $examplefile"
        sed -Ei 's~\{\{#example\s*?'${examplename}'\s*?\}\}~```rust,editable\n\{\{#include '$examplefile':example\}\}\n```~g' $file
        absoluteexampledir="${root}deps/tests/${rel}"
        # Create the folder for the test, if missing
        mkdir -p $absoluteexampledir
        # Add a stub file for the example
        cat > "$absoluteexampledir/${examplename}.rs" <<- 'EOF'
// ANCHOR: example
fn main() {
    // TODO
}
// ANCHOR_END: example

#[test]
#[ignore = "not yet implemented"]
fn test() {
    main();
}
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
