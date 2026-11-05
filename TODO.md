<!-- TODO Reviewed 10-25-2026 -->

# Task tracking

Open work is tracked in [GitHub Issues](https://github.com/john-cd/rust_howto/issues).
Keep one topic per issue, except epics, with a problem, task list, and acceptance criteria.
Record status and validation evidence in the issue rather than duplicating checklists here.

<!-- recommended-issue-order:start -->

### Blocked or deferred

- [#2116: Epic: waiting on an upstream release or advisory fix](https://github.com/john-cd/rust_howto/issues/2116) — Compatible fixed upstream dependencies; exact triggers in the issue.

#### Needs CI, hosts or services

- [#2207: Re-enable GitHub Actions and record hosted CI results](https://github.com/john-cd/rust_howto/issues/2207) — Billing/runner availability.
- [#2208: Service-backed ignored example tests](https://github.com/john-cd/rust_howto/issues/2208) — Real RabbitMQ, SQL Server, MySQL, Elasticsearch, SMTP, IMAP, etc.
- [#2209: Oracle ignored tests and fixed image](https://github.com/john-cd/rust_howto/issues/2209) — Registry credentials, Instant Client.
- [#2210: AWS-compatible and observability live checks](https://github.com/john-cd/rust_howto/issues/2210) — LocalStack/MinIO, OIDC provider, collector.
- [#2211: Python 3.15.2 devcontainer and Maturin/PyO3](https://github.com/john-cd/rust_howto/issues/2211) — Devcontainer build.
- [#2212: Feature-gated examples and heavy native stacks](https://github.com/john-cd/rust_howto/issues/2212) — Native SDKs and all-feature Clippy.
- [#2213: GUI, window and hardware examples](https://github.com/john-cd/rust_howto/issues/2213) — Display, screen reader, drone.
- [#2214: macOS and FreeBSD native examples](https://github.com/john-cd/rust_howto/issues/2214) — macOS/FreeBSD hosts.
- [#2202: Native Linux and macOS linker validation](https://github.com/john-cd/rust_howto/issues/2202) — Native hosts/CI.
- [#2203: Windows linker invocation and workspace matrix](https://github.com/john-cd/rust_howto/issues/2203) — Windows host.

#### Needs John

- [#2215: Maintainer decisions](https://github.com/john-cd/rust_howto/issues/2215) — Licensing, analytics, scope, tooling ownership.
- [#2216: Owner review of chapters, recipes and tools](https://github.com/john-cd/rust_howto/issues/2216) — Reviews; `TODO Reviewed` dates are not a blocker.

#### Tests and examples

- [#2206: Tests/examples for remaining tool-crate gaps](https://github.com/john-cd/rust_howto/issues/2206) — Tests/docs only.

<!-- recommended-issue-order:end -->

## Blocker tracking

- [Upstream release and advisory waits](https://github.com/john-cd/rust_howto/issues/2116).
- Host, service and human-dependent work: #2207-#2216 above.
