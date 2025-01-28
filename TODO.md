# TODOs

## Increase speed of CI build on Linux

### Try to move target and/or /usr/local/cargo to a docker volume instead of docker container FS

Metric:

```sh
time dd if=/dev/zero of=test.dat bs=1024 count=100000
```

Fast when writing to container FS, volume, but slow on bind mount (to Windows). Need to test on Linux

### Try to store target and/or /usr/local/cargo in a tmpfs?

/usr/local/cargo is small enough to fit in memory (16GB max)

<https://linuxhaxor.net/code/how-to-use-tmpfs-and-its-functions-in-docker.html>

### Try to build directly on the host, not in a container?

Rewrite the .yml file

## Make gha caching work on CI

Try local / inline caching as well

<https://cicube.io/blog/optimize-docker-builds-github-actions-cache/>

## Free space on CI runner

Already using the `free-disk-space-ubuntu` action

<https://github.com/actions/runner-images/issues/2875>

<https://github.com/marketplace/actions/free-disk-space-ubuntu>

## Create CI build on Windows

Then add build job to main workflow

## Add git hooks

Try `cargo-husky`

[dev-dependencies.cargo-husky]
version = "1.5.0"
default-features = false
features = ["user-hooks"]

Note that, when user-hooks feature is enabled, other all features are disabled.
You need to prepare all hooks in .cargo-husky/hooks directory
See <https://lib.rs/crates/cargo-husky>

See scripts/precommit folder

## CI build on MacOS too?

Create a .yml file for MacOS build on GitHub runner?
Add build job to main workflow
