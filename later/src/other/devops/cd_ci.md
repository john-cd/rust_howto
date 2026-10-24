<!-- TODO Reviewed ??-??-2026 -->
# Continuous Integration and Continuous Delivery / Deployment {#cdci}

{{#include cd_ci.incl.md}}

Continuous integration (CI) runs automated checks against proposed changes. Continuous delivery keeps accepted changes ready to release, while continuous deployment promotes them to production automatically after the required checks pass. Since teams use CD for either practice, document where a human approval or operational decision sits in your pipeline.

A pipeline should make a change traceable from its source commit through validation and a built artifact to each deployment. The same ideas apply across CI services; the [[github_actions | GitHub Actions]] chapter covers one provider in more detail.

## Continuous integration for Rust {#continuous-integration}

For each pull request, keep a fast, reliable set of required checks. A typical Rust starting point is:

~~~sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
~~~

Adapt package, target, feature, and operating-system coverage to the configurations your project supports. Use a toolchain and platform matrix when those are part of the compatibility promise; keep optional beta, nightly, or latest-dependency checks separate from required merge gates when they are exploratory. For applications or workspaces that commit `Cargo.lock`, pass `--locked` so dependency resolution cannot silently update it. See the [Cargo continuous integration guide][book~cargo~continuous-integration] for provider examples and additional tradeoffs.

## Deliver and deploy {#delivery-deployment}

After validation, build the release package and record its version, source commit, and checksum. Promote that same artifact through staging and production instead of rebuilding different bytes at each step. A deployment can then be rolled back or repeated using an artifact whose checks already passed.

For a library, release automation may publish a crate after its version and checks are verified. For an application, it may publish archives or container images, then deploy a selected version. Keep the publish or deploy job behind the appropriate tag, branch, approval, or environment rule rather than granting those credentials to every test job.

## Keep the pipeline safe and efficient {#safe-pipeline}

Use caches only to avoid repeating expensive downloads or compilation; a cache miss must not make a build fail. Use workflow artifacts for files produced by a run, such as test reports, release archives, or binaries passed between jobs. Do not put secrets in caches, and treat restored cache data as an optimization rather than a trusted release input. The [GitHub Actions artifact][github-actions~workflow-artifacts~docs] and [dependency caching][github-actions~dependency-caching~docs] guides explain the distinction.

Give each workflow or job only the permissions it needs. Do not expose deployment secrets to untrusted pull request code. Pin third-party actions to reviewed commit IDs, and prefer short-lived cloud credentials through OpenID Connect over long-lived cloud keys. Protected deployment environments can restrict eligible branches, require approval, and limit secret access; see GitHub's [security guidance][github-actions~security~docs] and [deployment environment documentation][github-actions~deployment-environments~docs].

## Example workflow from pull request to release {#github-actions}

In a GitHub Actions pipeline, pull requests can run formatting, linting, and tests without release credentials. A merge to the protected default branch can build and retain a candidate artifact. A version tag can publish that artifact, and a separate deploy job can promote it through protected environments. Define the exact triggers and approval points to match the project's release policy.

For an example of a mature Rust release workflow, see [ripgrep's release workflow][ripgrep~release-workflow]. For workflow syntax, tool installation, and caching examples, see [[github_actions | GitHub Actions]].

## Related resources {#related-resources .skip}

### Rust and CI services

- [Cargo's continuous integration guide][book~cargo~continuous-integration].
- [GitHub Actions][github-actions~website]↗, [GitLab CI/CD][gitlab~ci~docs], [Jenkins][jenkins~website]↗, and [CircleCI][circleci~website]↗.
- [Optimizing CI/CD pipelines for Rust projects][blog~optimizing-ci-cd-pipelines-rust-project]↗.

### Artifacts, caching, and deployment

- [GitHub workflow artifacts][github-actions~workflow-artifacts~docs].
- [GitHub dependency caching][github-actions~dependency-caching~docs].
- [GitHub deployment environments][github-actions~deployment-environments~docs].
- [GitHub Actions security][github-actions~security~docs].
- [Docker build caching with GitHub Actions][cache-management-with-github-actions~website]↗.

### Release example

- [ripgrep's release workflow][ripgrep~release-workflow].

## Related topics {#related-topics .skip}

- [[github_actions | GitHub Actions]].
- [[release_automation | Release Automation]].
- [[version_control | Version Control]].
- [[testing | Testing]].
- [[code_formatting_linting | Code Formatting and Linting]].
- [[cloud | Cloud]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[content audit: content removed by recent commits; review and restore](https://github.com/john-cd/rust_howto/issues/1904)
</div>
