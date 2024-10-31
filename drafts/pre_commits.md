## pre-commit

`pre-commit` is a framework for managing and maintaining pre-commit hooks.

Git hook scripts are useful for automatically identifying simple issues, such as missing semicolons, trailing whitespace, and debug statements, when commiting in `git`, before submission of the code to code review or a CI workflow.

### Installation

`pre-commit` is written in Python. Include the following into your `Dockerfile` or run the commands by hand to install `pre-commit`:

```sh
## Install python3, pipx, pre-commit (Ubuntu & friends)
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y python3 pipx \
    && pipx install pre-commit \
    && pipx ensurepath
```

```sh
pre-commit --version
```

- Add a file called `.pre-commit-config.yaml` to the root of your project. Use `pre-commit sample-config` for a template.
- Edit it to configure your preferred hooks.

```sh
# Set up the git hook scripts
pre-commit install

# It's usually a good idea to run the hooks against all of the files when adding new hooks
# (pre-commit will only run on the changed files during git hooks)
pre-commit run --all-files
```

## Useful links

https://rodneylab.com/rust-ci-tooling/

https://pre-commit.com/

https://pre-commit.com/hooks.html

https://github.com/doublify/pre-commit-rust

https://github.com/alessandrojcm/commitlint-pre-commit-hook

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO write
identify useful hooks for Rust
- cargo fmt
- cargo check, clippy, test...
- spell checks
add a sample config
</div>
