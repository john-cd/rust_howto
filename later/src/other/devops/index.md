<!-- TODO Reviewed ??-??-2026 -->

# DevOps and Rust

DevOps connects software development and operations through practices for version control, automated checks, packaging, deployment, and service operation. This index links to the Rust-specific guides and highlights infrastructure tools with Rust support.

## Start Here {#start-here}

| Guide                                       | Focus                                                                       |
| ------------------------------------------- | --------------------------------------------------------------------------- |
| [Version Control](version_control.md)       | Review changes, manage branches, and keep development workflows repeatable. |
| [CI/CD](cd_ci.md)                           | Run automated checks and promote verified artifacts.                        |
| [GitHub Actions](github_actions.md)         | Configure Rust toolchains, caches, and workflows on GitHub.                 |
| [Release Automation](release_automation.md) | Version, validate, package, and publish a release.                          |
| [Containers][p~containers]                  | Build and operate containerized applications.                               |
| [Using Containers][p~using-containers]      | Use Rust clients to work with container runtimes.                           |

## Infrastructure as Code {#infrastructure-as-code}

Infrastructure as code (IaC) describes infrastructure in files that can be reviewed, versioned, and applied by a deployment tool. Rust support differs by project:

- [Pulumi's Rust SDK][infra~pulumi-rust-status]↗ was released by Pulumi Labs as an **experimental** SDK and language plugin in August 2026. It is early-stage, not officially supported, and its API is still evolving.
- [KCL][infra~kcl-lang-repo]↗ is a cloud-native configuration and policy language whose core is written in Rust. The project also publishes [Rust bindings][infra~kcl-rust-bindings]↗; KCL programs use the KCL language rather than Rust syntax.
- [Terraform provider development][infra~terraform-provider-language]↗ is officially supported by HashiCorp in Go. Writing providers in another language is technically possible at the protocol level, but HashiCorp does not support the tooling or provider development workflow for non-Go languages.

## Configuration Management {#configuration-management}

Configuration management applies repeatable settings and lifecycle behavior to hosts and services. [Chef Habitat][infra~habitat-repo]↗ packages applications with their runtime and lifecycle hooks and includes a Supervisor implemented in Rust. Its [official release notes][infra~habitat-releases]↗ continue to document releases in 2026. Habitat is focused on packaging and running applications, rather than general-purpose operating-system configuration.

## Containerization and Orchestration {#containerization-and-orchestration}

Use the [Containers][p~containers] guide for container concepts. For Kubernetes applications, [`kube`][infra~kube-docs]↗ provides a Rust client, custom-resource support, and a controller runtime.

For Docker or Podman API access from Rust, see [Bollard][infra~bollard-repo]↗, an asynchronous client library. The [Using Containers][p~using-containers] guide develops practical examples. The older `docker-api` bookmark has been replaced with this maintained project.

## Related Topics {#related-topics .skip}

- [Configuration][p~configuration] for configuration inside Rust applications.
- [Testing][p~testing] and [Cargo auditing][p~maintaining] for checks that run before release.
- Deployment to cloud platforms and container orchestration systems.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
