default:
  @just --list --unsorted

clean:
  cargo clean
  mdbook clean

build:
  cargo build
  mdbook build

serve: test
  mdbook serve --open
  # to change the port: --port 3001

test:
  mdbook test

update:
  cargo update

