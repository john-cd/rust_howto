## Check external command version for compatibility

[![semver-badge]][semver] [![cat-text-processing-badge]][cat-text-processing] [![cat-os-badge]][cat-os]

Runs `git --version` using [`Command`], then parses the version number into a
[`semver::Version`] using [`Version::parse`]. [`VersionReq::matches`] compares
[`semver::VersionReq`] to the parsed version.  The command output resembles
"git version x.y.z".

```rust,editable,no_run
{#include ../../../deps/examples/semver-command.rs}
```

[`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
[`semver::Version`]: https://docs.rs/semver/*/semver/struct.Version.html
[`semver::VersionReq`]: https://docs.rs/semver/*/semver/struct.VersionReq.html
[`Version::parse`]: https://docs.rs/semver/*/semver/struct.Version.html#method.parse
[`VersionReq::matches`]: https://docs.rs/semver/*/semver/struct.VersionReq.html#method.matches
