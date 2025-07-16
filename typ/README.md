# README

## Install `typst`

```sh
# If snap is not installed:
sudo apt update
sudo apt install snapd

sudo snap install typst
```

## Use `typst`

```sh
# Create `file.pdf` in working directory:
typst compile file.typ
# Create PDF file at the desired path:
typst compile path/to/source.typ path/to/output.pdf
```

### Watches source files and recompiles on changes

```sh
typst watch file.typ
```

```sh
typst help
# Prints detailed usage of a subcommand
typst help watch
```

## Packages

<https://typst.app/universe/package/codly>

## Templates

<https://github.com/typst/templates/tree/main/wonderous-book>

## References

[Exploring Typst, a new typesetting system similar to LaTeX](https://blog.jreyesr.com/posts/typst)

See also: <https://asciidoc.org/>
