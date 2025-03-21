## ---- DEFAULT -------------------------------------

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

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
    pre-commit run prettier    # fixes all Markdown (& other) lint issues

# FIXME Install/update code automation: pre-commit...
@install:
    curl https://pre-commit.com/install-local.py | python3 -
#npm i -g prettier
# [move to Dockerfile](https://github.com/john-cd/rust_howto/issues/1247)

# FIXME Setup/update pre-commit hooks (optional)
@setup:
    pre-commit install --install-hooks # uninstall: `pre-commit uninstall`

## ---- GIT TOWN -----------------------------------------

# [review git town](https://github.com/john-cd/rust_howto/issues/1246)
# https://www.git-town.com/ (setup receipt)

# Configure `git town`
@config_git_town:
    git config git-town.code-hosting-driver gitea  # setup git-town with gitea
    git config git-town.code-hosting-origin-hostname gitea.example.org  # setup git-town origin hostname
