<!-- TODO Reviewed ??-??-2026 -->

# Networking Utilities

{{#include networking_cli.incl.md}}

## Send HTTP Requests {#http-requests}

[`xh`][xh~repo]↗ is a fast, friendly HTTP client with an interface inspired by HTTPie.

```bash
cargo install xh --locked
xh https://httpbin.org/json
```

## Load Test HTTP Services {#http-load-testing}

[`oha`][oha~repo]↗ sends concurrent HTTP requests and reports latency and throughput statistics. Load tests consume server resources; run them only against services you own or have permission to test.

```bash
cargo install oha --locked
oha -n 100 -c 10 http://localhost:3000/
```

## Monitor Local Bandwidth {#bandwidth-monitoring}

[`bandwhich`][bandwhich~repo]↗ shows network use by process. The project is in passive maintenance: critical issues are addressed, but new features are not being developed. Packet capture requires elevated privileges on Linux and a supported packet-capture driver on Windows; consult the project instructions for your platform.

```bash
cargo install bandwhich
sudo bandwhich
```

## Diagnose Connectivity {#connectivity-diagnostics}

[`gping`][c~gping~repo]↗ graphs ping latency. [`trippy`][trippy~repo]↗ combines ping and traceroute to help inspect the route and packet loss to a host.

### `gping` {#gping}

```bash
cargo install gping --locked
gping example.com
```

### `trippy`

```bash
cargo install trippy --locked
sudo trip example.com
```

Raw-socket permissions may be required. See Trippy's installation and privilege guidance for platform-specific setup.

## Query DNS {#dns-queries}

[`doggo`][doggo~repo]↗ is a maintained DNS client with support for encrypted DNS protocols. It is written in Go, not Rust; the original `dog` client is Rust. Use `doggo` when you want the maintained tool and do not require a Rust implementation.

```bash
go install github.com/mr-karan/doggo/cmd/doggo@latest
doggo MX example.com @1.1.1.1
```

## Scan Authorized Hosts {#port-scanning}

[`rustscan`][c~rustscan~repo]↗ scans ports and can hand discovered ports to Nmap. Port scanning can disrupt systems or violate policy. Scan only hosts you own or are explicitly authorized to assess, and follow the target network's rules.

```bash
cargo install rustscan --locked
rustscan -a 192.0.2.10
```

Replace the example IP address with a host you own or have explicit permission to scan.

## Related Topics {#related-topics .skip}

- [[command-line-interface | Command Line Interface]].
- [[external_commands | External Commands]].
- [[network-programming | Network Programming]].
- [HTTP clients](../web-programming_http-client/requests.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
