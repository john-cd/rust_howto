# TODOs

## Increase speed of CI build

Try to move target and/or /usr/local/cargo to a docker volume? to tempfs?

Metric:

```sh
time dd if=/dev/zero of=test.dat bs=1024 count=100000
```

Fast when writing to container FS, volume, but slow on bind mount (to Windows). Need to test on Linux

## Free space on CI runner

Already using the `free-disk-space-ubuntu` action

<https://github.com/actions/runner-images/issues/2875>

<https://github.com/marketplace/actions/free-disk-space-ubuntu>

## Create CI build on Windows

## Add git hooks

Try `cargo-husky`

[dev-dependencies.cargo-husky]
version = "1.5.0"
default-features = false
features = ["user-hooks"]

Note that, when user-hooks feature is enabled, other all features are disabled.
You need to prepare all hooks in .cargo-husky/hooks directory
See <https://lib.rs/crates/cargo-husky>
