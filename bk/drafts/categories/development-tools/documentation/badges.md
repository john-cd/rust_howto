<!-- TODO Reviewed ??-??-2026 -->

# Badges {#badges}

{{#include badges.incl.md}}

[![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

A badge{{hi:Badges}} is a small image that displays status information about a crate, for example the version number or build status, and links to a detailed page.

[Shields.io][shield-io~website]↗{{hi:shields.io}} generates badges from a range of services. Replace `CRATE` and `OWNER/REPO` in these examples with your crate name and repository. Link each badge to a page where readers can verify its status.

## Crates.io Version

```markdown
[![Crates.io version](https://img.shields.io/crates/v/CRATE)](https://crates.io/crates/CRATE)
```

## Documentation

```markdown
[![Documentation](https://docs.rs/CRATE/badge.svg)](https://docs.rs/CRATE)
```

## License

```markdown
[![License](https://img.shields.io/crates/l/CRATE)](https://crates.io/crates/CRATE)
```

## Continuous Integration

```markdown
[![CI](https://github.com/OWNER/REPO/actions/workflows/ci.yml/badge.svg)](https://github.com/OWNER/REPO/actions/workflows/ci.yml)
```

## Downloads

```markdown
[![Downloads](https://img.shields.io/crates/d/CRATE)](https://crates.io/crates/CRATE)
```

## Minimum Supported Rust Version

```markdown
[![MSRV](https://img.shields.io/crates/msrv/CRATE)](https://crates.io/crates/CRATE)
```

## Coverage

```markdown
[![Coverage](https://img.shields.io/codecov/c/github/OWNER/REPO)](https://app.codecov.io/gh/OWNER/REPO)
```

Check the service's current badge URL and repository settings before adding a badge. A badge reports only the state its source service exposes; it does not replace release, CI, or coverage checks.

Crate badges use links generated from the repository's central crate-reference data. See the [contributor guide to repository structure](../../../../src/appendices/contributing/repository_structure.md#folders-and-key-files) for the reference generation workflow; update its source data instead of copying badge URLs into chapter references.

See the [Shields.io badge catalog][shields~badge-reference~docs], [docs.rs badge guidance][docsrs~badge~docs], and [GitHub workflow status badge documentation][github-actions~status-badge~docs] for current service options.

## Related Topics {#related-topics .skip}

- [Documenting your code][ex~documentation~documenting-your-code].
- [Cargo packages][p~cargo] and crate metadata.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
