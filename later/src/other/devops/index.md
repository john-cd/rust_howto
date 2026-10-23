<!-- TODO Reviewed ??-??-2026 -->
# DevOps and Rust

DevOps connects software development and operations through collaboration, automation, and continuous improvement. Common practices and example tools include:

| Practice | Purpose | Examples |
|---|---|---|
| Version control | Track changes, review contributions, and recover earlier versions. | Git, GitHub, GitLab, Bitbucket |
| Continuous integration and delivery | Build and test changes, then automate release or deployment steps. | GitHub Actions, Jenkins, GitLab CI/CD, Azure Pipelines |
| Infrastructure as code | Describe and version infrastructure so it can be provisioned consistently. | Terraform, OpenTofu, AWS CloudFormation, Pulumi |
| Configuration management | Apply consistent operating-system and application settings. | Ansible, Puppet, Chef |
| Containers and orchestration | Package applications with their runtime dependencies and coordinate deployment. | Docker, Kubernetes, Amazon ECS |
| Monitoring and logging | Collect signals that help operators understand service health and investigate failures. | Prometheus, Grafana, OpenTelemetry, Elasticsearch |
| Security in delivery | Check dependencies, credentials, artifacts, and deployment permissions throughout delivery. | `cargo audit`, `cargo deny`, secret managers, signing tools |

Rust coders, like developers in other fields, frequently leverage DevOps methods and tooling. GitHub, GitHub Actions, and Docker are especially frequently used in Rust open-source projects.

## Version Control

{{#include version_control.incl.md}}

## CD/CI

CI/CD stands for Continuous Integration and Continuous Delivery/Deployment. It's a set of practices that automate the software development lifecycle, aiming to deliver code changes more frequently and reliably.

{{#include cd_ci.incl.md}}

{{#include github_actions.incl.md}}

### Release Automation

{{#include release_automation.incl.md}}

## Infrastructure as Code

Describe infrastructure declaratively and keep its configuration under version control. See [[cloud | Cloud]] for Rust on cloud platforms.

## Configuration Management Tools

Use configuration management to apply repeatable host and service settings. See [[config | Configuration]] for application configuration in Rust.

## Containerization and Orchestration

See [[containers | Containers]].

## Devops Tools Written in Rust

Rust is also commonly being used to build DevOps tools. A few examples follow:

- Chef's [`Habitat`][chef-habitat~website]↗{{hi:Habitat}} is a Rust-based infrastructure automation and management tool. Chef Habitat bundles all of your dependencies, codebases, and lifecycle events in a package separate from the user's operating system.
- Faraday used Rust to develop its `Cage` project. `Cage` is most useful to reuse existing Docker images. You can load existing images into your local environment and start adding new components from local source trees.
- [`sccache`][c~sccache~docs]↗{{hi:sccache}} is a [[build_cache | cache]]. compiler tool that can automatically package local toolchains. It supports Rust and C/C++.
- [`config-rs`][c~config~docs]↗{{hi:config-rs}} is a layered [[config | configuration]] system for Rust applications.

Refer to the [[written-in-rust | "Written in Rust"]] chapter as well.

## Related Topics

- [[config | Config]].
- [[environment_variables | Environment Variables]].
- Deployment on [[cloud | Cloud]] platforms ([[aws | AWS]], Azure, GCP).
- Unit, integration, and end-to-end [[testing | tests]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/605)
- Infrastructure as Code.
- Configuration Management Tools.
- Containerization and Orchestration.

Review:

- [Chef Habitat][chef-habitat~website] [(GitHub)][chef-habitat~repo].
- [KCL Programming Language][kcl-lang~website] [(GitHub)][kcl-lang~repo].
- [`docker-api`][c~docker-api~docs]↗{{hi:docker-api}}: a Rust interface to Docker containers.
- [Optimizing DevOps Pipelines for Rust Projects][blog~optimizing-devops-pipelines-for-rust-projects-leveraging-cargo-and-cicd]↗: Leveraging Cargo and CI/CD.

CD/CI:

- [`gh-workflow`][c~gh-workflow~crates.io]↗.
- [GitHub Actions best practices for Rust projects][blog~github-actions-best-practices]↗.
- [`pipelight`][pipelight~repo]↗: Tiny automation pipelines. Bring CI/CD to the smallest projects. Self-hosted, Lightweight, CLI only.

</div>
