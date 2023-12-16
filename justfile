# list of the (last part of) folder names under the `xmpl` directory, space separated
examples := `find ./xmpl -mindepth 1 -maxdepth 1 -type d | awk -F'/' '{print $(NF)}' | tr '\n' ' '`

default:
  @just --list --unsorted
# or: @just --choose

# Clean Cargo's `target` and mdbook's `book` and `doctest_cache` directories
clean:
  cargo clean
  mdbook clean
  rm --recursive --force ./doctest_cache/

# Format the code of all projects in the xmpl folder (DOES NOT INCLUDE examples embedded in the markdown)
xfmt:
  for d in {{examples}}; do ( echo $d; cargo fmt -v --package $d ); done

# Scan the code of all projects in the xmpl folder for common mistakes (DOES NOT INCLUDE examples embedded in the markdown)
xclippy:
  for d in {{examples}}; do ( echo $d; cargo clippy --package $d ); done

# Check all projects in the xmpl folder (and all of their dependencies) for errors
xcheck:
  for d in {{examples}}; do ( echo $d; cargo check --package $d ); done
#  cargo check --workspace --all-targets --all-features

# Compile all projects in the xmpl folder
xbuild:
  for d in {{examples}}; do ( echo $d; cargo build --package $d ); done
#  cargo build --workspace --all-targets --all-features

# Test all projects
#xtest:
#  cargo test --workspace --all-targets --all-features

# Build the book from its markdown files (incl. testing of the examples embedded in the markdown)
build:
  mdbook build

# Test the examples embedded in the markdown
test:
  mdbook test
#  mdbook test --library-path /cargo-target-rust_howto/target/debug/deps/
# see: https://doc.rust-lang.org/rustdoc/command-line-arguments.html#-l--library-path-where-to-look-for-dependencies

# Serve the book (incl. testing of the examples embedded in the markdown)
serve:
  mdbook serve --open
  # to change the port: --port 3001

# Watch the book's markdown files and rebuilds it on changes
# watch:
#   mdbook watch --open

# Update Cargo.lock dependencies for all projects (incl. dependencies used by the book and the xmpl folder)
update:
  cargo update
