default:
  @just --list --unsorted

clean:
  mdbook clean

build:
  mdbook build

serve:
  mdbook serve --open # --port 3001




