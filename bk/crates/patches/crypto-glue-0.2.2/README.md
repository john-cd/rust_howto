<!-- TODO Reviewed ??-??-2026. -->

# Local `crypto-glue` compatibility patch

This directory carries the `crypto-glue` 0.2.2 source from
[https://github.com/kanidm/crypto-glue](https://github.com/kanidm/crypto-glue) for the `bk/crates` workspace.

The local manifest removes the obsolete `rsa` `sha2` feature request. Stable
`rsa` 0.10 removed that feature, while `crypto-glue` already depends directly
on `sha2`. Remove this patch when an upstream release resolves the mismatch.

The source and local modifications are available under the Mozilla Public
License 2.0. See the accompanying [license text](LICENSE).
