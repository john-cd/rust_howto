#! /usr/bin/env bash

set -e

# mds=$(find ../src -type f -name "*.md" | tr '\n' ' ')

## all files that include a rust code block
mds=()
# -r = recursvive -P = Perl -l = output file path -z = process everything, not line by line
for file in $(grep -r -l -P -z '(?s)```rust.*```' ../src/ )
do
  mds+=($( realpath $file ))
done

#echo $mds

rm -rf ./_temp/
[ ! -d ./_temp/ ] && mkdir ./_temp/
cargo init --lib --vcs none ./_temp/
cargo add doc-comment --manifest-path ./_temp/Cargo.toml

[ ! -d ./_temp/src ] && mkdir -p ./_temp/src

cat <<EOF > ./_temp/src/lib.rs
#![allow(non_snake_case)]
#![allow(unused)]

#[macro_use]
extern crate doc_comment;

mod tests;

EOF

for doc in ${mds[@]}
do
    #echo $doc
    NAME=$(basename $doc .md)
    NAME=${NAME//./_}
    NAME=${NAME//-/_}
    #echo $NAME
    echo -e "mod _$NAME {\ndoctest\041(\"$doc\"); \n}\n\n" >> ./_temp/src/tests.rs
done

# Remove any rust code blocks that use {{#include ... }}
cd _temp
cargo build
## Grep lines that do not have {{#include ... }}, no file path output
cargo expand > ./src/expanded.rs

cat ./src/expanded.rs | rg --multiline '(?s)```rust.*?```' >| ./src/examples.md

sed -i -e 's/```/\n```/g' ./src/examples.md

cat ./src/examples.md | rg --multiline '(?s)```rust.*?ignore.*?```' >| ./src/ignored.md

cat ./src/expanded.rs | rg --invert-match '\{\{#include.*\}\}.*' >> ./src/lib.rs

sed -i -e 's/mod tests;//g' -e 's/#[prelude_import]//g' ./src/lib.rs

# | tee /dev/stderr
# TODO --multiline '(?s)```rust.*?\{\{#include.*?\}\}.*?```'


# Reference: https://docs.rs/doc-comment/latest/doc_comment/
# Thanks to https://github.com/rust-random/book/blob/master/tests/generate.sh
# See https://github.com/rust-lang/mdBook/issues/706

# In GitHub Action workflow, add
#       - name: Generate harness
#         working-directory: ./tests
#         run: ./generate.sh
#       - name: Test code samples
#         working-directory: ./tests
#         run: cargo test
