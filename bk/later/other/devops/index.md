# DevOps and Rust

DevOps is the integration and automation of the software development and information technology operations. It prioritizes automation, collaboration, and continuous improvement. The following is a brief description of key DevOps practices and common tools:

| Key DevOps Practices | Description | Technologies (examples) |
| Version Control | Tracking changes. Undoing mistakes easily. Protecting against data loss. | Git, GitHub, GitLab, Bitbucket. |
| CI/CD | Automating code testing and the release process. Automatically deploying changes to production. | Jenkins, GitLab CI/CD, CircleCI, Azure DevOps, AWS CodePipeline. |
| Infrastructure as Code | Managing infrastructure through code. Enabling automation and version control of infrastructure. | Terraform, Open Tofu, AWS CloudFormation, Pulumi. |
| Configuration Management Tools | Automating the configuration of systems and software. Ensuring consistency and repeatability. | Ansible, Puppet, Chef. |
| Containerization and Orchestration | Packaging an application and its dependencies into a single, portable unit called a container. | Docker, Kubernetes, Amazon ECS. |
| Monitoring and Logging Tools | Collecting and analyzing data to understand system performance. Proactive identification and resolution of issues. | Prometheus, Grafana, ELK stack (Elasticsearch, Logstash, Kibana), Splunk, DataDog. |
| Cloud Computing | On-demand delivery of computing services over the Internet. | AWS, Azure, Google Cloud Platform (GCP). |
| DevSecOps | Integrating security practices into every stage of the DevOps lifecycle. | Snyk, HashiCorp Vault, AWS Secrets Manager, Splunk |

Rust coders, like developers in other fields, frequently leverage DevOps methods and tooling. GitHub, GitHub Actions, and Docker are especially frequently used in Rust open-source projects.

## Version Control

{{#include version_control.incl.md}}

## CD/CI

CI/CD stands for Continuous Integration and Continuous Delivery/Deployment. It's a set of practices that automate the software development lifecycle, aiming to deliver code changes more frequently and reliably.

{{#include ci_cd.incl.md}}

{{#include github_actions.incl.md}}

### Release Automation

{{#include release_automation.incl.md}}

## Infrastructure as Code

## Configuration Management Tools

## Containerization and Orchestration

See [[containers | Containers]].

## Devops Tools Written in Rust

Rust is also commonly being used to build DevOps tools. A few examples follow:

- Chef's [`Habitat`][chef-habitat~website]↗{{hi:Habitat}} is a Rust-based infrastructure automation and management tool. Chef Habitat lets you bundle all of your dependencies, codebases, and lifecycle events in a package separate from the user's operating system.
- Faraday used Rust to develop its `Cage` project. `Cage` is most useful when you want to reuse existing Docker images. You can load existing images into your local environment and start adding new components from local source trees.
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

- [Chef Habitat][chef-habitat~website] [(GitHub)][chef-habitat~github].
- [KCL Programming Language][kcl-lang~website] [(GitHub)][kcl-lang~github].
- [`docker-api`][c~docker-api~docs]↗{{hi:docker-api}}: a Rust interface to Docker containers.
- [Optimizing DevOps Pipelines for Rust Projects][blog~optimizing-devops-pipelines-for-rust-projects-leveraging-cargo-and-cicd]↗: Leveraging Cargo and CI/CD.

CD/CI:

- [`gh-workflow`][c~gh-workflow~crates.io]↗.
- [GitHub Actions best practices for Rust projects][blog~github-actions-best-practices]↗.
- [`pipelight`][pipelight~github]↗: Tiny automation pipelines. Bring CI/CD to the smallest projects. Self-hosted, Lightweight, CLI only.

</div>
