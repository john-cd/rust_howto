#! /bin/bash

# Create a crate for each category

set -euo pipefail

root="/code/"

for dir in $(find ${root}crates/ex/categories/ -type d -wholename "*/tests/*" )
do
  cat=$(basename $dir)
  path="${root}crates/ex/categories/${cat}"
  if [ ! -d "${dir}" ]; then
    echo ">> ${path}"
    cargo new --lib ${path}
    echo "# README" > ${path}/README.md
    mkdir -p ${path}/tests
    mkdir -p ${path}/temp
    touch ${path}/temp/.gitkeep
    cat > ${path}/.gitignore <<-EOF
temp/
!temp/.gitkeep
target/
EOF
    cp /code/crates/ex/categories/ab/LICENSE ${path}/LICENSE
    cat > ${path}/src/lib.rs <<-EOF
//! Should remain empty
EOF
    mv -n -t ${path}/tests ${dir}/*
    #echo "mod ${cat};" > ${path}/tests/cat.rs

    sed -i 's/description.workspace = true/description = "Book code examples and their crate dependencies - '${cat}'"/' ${path}/Cargo.toml
    sed -i 's/license-file.workspace = true/license = "CC0-1.0"/' ${path}/Cargo.toml
    sed -i 's/categories.workspace = true/categories = [ "'${cat}'" ]/' ${path}/Cargo.toml
    break
  else
    echo "${path} exists!"
  fi
done
