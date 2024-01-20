## Find the latest version satisfying given range

[![semver-badge]][semver] [![cat-config-badge]][cat-config]

Given a list of version &strs, finds the latest [`semver::Version`].
[`semver::VersionReq`] filters the list with [`VersionReq::matches`].
Also demonstrates `semver` pre-release preferences.

```rust,editable
{#include ../../../deps/examples/semver-latest.rs}
```

[`semver::Version`]: https://docs.rs/semver/*/semver/struct.Version.html
[`semver::VersionReq`]: https://docs.rs/semver/*/semver/struct.VersionReq.html
[`VersionReq::matches`]: https://docs.rs/semver/*/semver/struct.VersionReq.html#method.matches
