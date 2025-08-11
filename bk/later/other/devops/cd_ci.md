# Continuous Deployment / Continuous Integration {#cdci}

{{#include cd_ci.incl.md}}

CI/CD (Continuous Integration/Continuous Delivery or Deployment) systems are essential for automating the release process.

Popular tooling include:

- `GitHub Actions`.
- `GitLab CI/CD`.
- [`Jenkins`][jenkins~website]↗{{hi:Jenkins}}.
- [`CircleCI`][circleci~website]↗{{hi:CircleCI}}.

## Workflow Example (GitHub Actions) {#github-actions}

- Developers push code changes to a Git repository.
- [`GitHub Actions`][github-actions~website]{{hi:GitHub Actions}} is triggered by the push or pull request.
- Within the workflow, [`cargo`][c~cargo~docs]↗{{hi:cargo}} builds the Rust application in release mode.
- [`cargo`][c~cargo~docs]↗{{hi:cargo}} runs all (unit, integration, and end-to-end) tests.
- The built binary is packaged (e.g., zipped).
- [`GitHub Actions`][github-actions~website]{{hi:GitHub Actions}} creates a new release on GitHub, attaching the packaged binary.
- Deployment (Optional): [`GitHub Actions`][github-actions~website]{{hi:GitHub Actions}} deploys the application to a target environment.

See [[github_actions | Github Actions]].

## Related Topics {#related-topics}

[Continuous Integration (cargo book)][book~cargo~continuous-integration]↗.

- [Optimizing CI/CD pipelines][blog~optimizing-ci-cd-pipelines-rust-project]↗.
- [Creating a docker container action][github~creating-a-docker-container-action]↗.
- ["Upload a Build Artifact" Github Action][upload-a-build-artifact~website]↗.
- [`docker-cache`][docker-cache~github]{{hi:docker-cache}}↗.
- [Cached Docker images][cached-docker-images~github]↗.
- [Docker GitHub Action][docker~github-action]↗.
- [Cache storage backends][docker~cache-storage-backends]↗.
- [Cache management with GitHub Actions][cache-management-with~github-actions~website]↗.
- [How to cache docker-compose build inside github-action][stackoverflow~how-to-cache-docker-compose-build-inside~github-action]↗.
- [Optimizing CI/CD pipelines in your Rust projects][blog~optimizing-ci-cd-pipelines-rust-project]↗.
- [GitHub Action for installing development tools (mainly from GitHub Releases)][install_action~github]↗.
- [Experience about deploying [`mdbook`][c~mdbook~docs]↗{{hi:mdbook}} on github using github action - tutorials - The Rust Programming Language Forum][forum-deploying-mdbook-on~github-using~github-action]↗.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[cd_ci: write; organize links](https://github.com/john-cd/rust_howto/issues/595)
example: https://github.com/BurntSushi/ripgrep/blob/master/.github/workflows/release.yml
</div>
