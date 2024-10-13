# Docker Compose

An easy way to get started is to run `docker init` in a new folder, select `Rust` as the project type, then edit the provided `compose.yaml` and `Dockerfile`.

```bash
cargo init .
docker init
```

- Build your image: `docker build -t myapp .`.
- If your cloud uses a different CPU architecture than your development machine (e.g., you are on a Mac M1 and your cloud provider is amd64), you'll want to build the image for that platform, e.g.: `docker build --platform=linux/amd64 -t myapp .`.
- Start your application by running: `docker compose up --build`

## References

- Docker's [getting started][docker-getting-started]⮳ docs.
- [Docker's Rust guide][docker-rust-guide]⮳

{{#include ../refs/link-refs.md}}
