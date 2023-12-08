default:
  @just --list --unsorted

clean:
  mdbook clean

build:
  mdbook build

serve: test
  mdbook serve --open
  # to change the port: --port 3001

test:
  mdbook test



