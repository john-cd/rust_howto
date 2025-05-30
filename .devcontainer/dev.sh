#!/usr/bin/env bash

# Script executed at the end of the development Docker image build.
# Format the book code, then fetch the dependencies of the book code.

# NO set -e on purpose
set -x
set -v

## Check whether Docker has failed to bind mount the development laptop's project folder to /code
## You may also verify that the bind mount was created correctly using `docker inspect <container_name>`. Look for the Mounts section.
## Bind mount failures may happen with Dev Drives under Win 11. In this case, store your code in WSL e.g. ~/linuxcode/rust_howto/
## and then use `code .` from the WSL prompt in that folder.
if [ ! -d /code/.devcontainer/ ]; then
  echo "ERROR: CHECK BIND MOUNT"
  exec "$@"
  exit 111
fi

echo "-------------------------------------------"

cd ./bk/crates

echo "Format the code"
cargo +nightly fmt --all

echo "Fetch the dependencies"
cargo fetch

echo "Toolchain info:"
rustup check
echo "-------------------------------------------"

# Do not remove.
# This is what will run any dockerfile CMD.
exec "$@"

exit 0
