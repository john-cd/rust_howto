## ---- DEFAULT -------------------------------------

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ source_file() }}

## ---- GIT -----------------------------------------
## Commit, skipping pre-commit hooks
#@commit m:
#    git commit --no-verify -m "{{m}}"
## Amend the last commit, skipping pre-commit hooks
#@amend:
#    git commit --amend --no-verify
## ---- PRE-COMMIT -----------------------------------
# [finish - install pre-commit or use cargo-husky](https://github.com/john-cd/rust_howto/issues/1245)?
# Combining just + pre-commit
# pre-commit: https://pre-commit.com/
# based on https://github.com/casey/just/blob/master/examples/pre-commit.just
# [review](https://github.com/john-cd/rust_howto/issues/1246)
# https://github.com/prettier/prettier (pre-commit hook)

# FIXME Fix auto-fixable lint issues in staged files
@fix:
    pre-commit run prettier # fixes all Markdown (& other) lint issues

# FIXME Install/update code automation: pre-commit...
[unix]
@install:
    curl https://pre-commit.com/install-local.py | python3 -

[windows]
@install:
    py -3 -m pip install pre-commit

#npm i -g prettier
# [move to Dockerfile](https://github.com/john-cd/rust_howto/issues/1247)

# FIXME Setup/update pre-commit hooks (optional)
@setup:
    pre-commit install --install-hooks # uninstall: `pre-commit uninstall`

## ---- GIT TOWN -----------------------------------------
# [review Git Town configuration](https://github.com/john-cd/rust_howto/issues/1246)
# Optional repository-local configuration for Git Town (https://www.git-town.com/).

# Configure Git Town to use this repository's GitHub remote
@config_git_town:
    git config git-town.forge-type github
    git config git-town.hosting-origin-hostname github.com
