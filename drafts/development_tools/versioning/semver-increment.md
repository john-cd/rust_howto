## Parse and increment a version string

[![semver-badge]][semver] [![cat-config-badge]][cat-config]

Constructs a [`semver::Version`] from a string literal using [`Version::parse`],
then increments it by patch, minor, and major version number one by one.

Note that in accordance with the [Semantic Versioning Specification],
incrementing the minor version number resets the patch version number to 0 and
incrementing the major version number resets both the minor and patch version
numbers to 0.

```rust,editable
{#include ../../../deps/examples/semver-increment.rs}
```

[`semver::Version`]: https://docs.rs/semver/*/semver/struct.Version.html
[`Version::parse`]: https://docs.rs/semver/*/semver/struct.Version.html#method.parse

[Semantic Versioning Specification]: http://semver.org/
