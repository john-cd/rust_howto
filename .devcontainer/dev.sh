#!/usr/bin/env bash

# NO set -e on purpose
set -x
set -v

if [ ! -d /code ]; then
  echo "/code is not accessible somehow. CHECK BIND MOUNT."
  exec "$@"
  exit 0
fi

echo "-------------------------------------------"
echo "Format the code"
just fmtall

echo "Fetch the dependencies"
cargo fetch

echo "Build the book"
just build

echo "Build code"
just buildall

echo "Toolchain info:"
rustup check
echo "-------------------------------------------"

# Do not remove.
# This is what will run any dockerfile CMD.
exec "$@"

exit 0
