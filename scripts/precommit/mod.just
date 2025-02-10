## ---- DEFAULT -------------------------------------

root := justfile_directory()

_default:
    @just --list precommit --unsorted --justfile {{justfile()}}

## ---- GIT -----------------------------------------

## Commit, skipping pre-commit hooks
#@commit m:
#    git commit --no-verify -m "{{m}}"

## Amend the last commit, skipping pre-commit hooks
#@amend:
#    git commit --amend --no-verify

## ---- PRE-COMMIT -----------------------------------

# TODO P1 finish - install pre-commit or use cargo-husky?
# Combining just + pre-commit
# pre-commit: https://pre-commit.com/
# based on https://github.com/casey/just/blob/master/examples/pre-commit.just

# TODO review
# https://github.com/prettier/prettier (pre-commit hook)

# WIP Fix auto-fixable lint issues in staged files
@fix:
    pre-commit run prettier    # fixes all Markdown (& other) lint issues

# WIP Install/update code automation: pre-commit...
@install:
    curl https://pre-commit.com/install-local.py | python3 -
#npm i -g prettier
# TODO move to Dockerfile

# WIP Setup/update pre-commit hooks (optional)
@setup:
    pre-commit install --install-hooks # uninstall: `pre-commit uninstall`

## ---- GIT TOWN -----------------------------------------

# TODO P2 review git town
# https://www.git-town.com/ (setup receipt)

# WIP configure git town
@config_git_town:
    git config git-town.code-hosting-driver gitea  # setup git-town with gitea
    git config git-town.code-hosting-origin-hostname gitea.example.org  # setup git-town origin hostname
