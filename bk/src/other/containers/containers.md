# Rust + containers

{{#include containers.incl.md}}

## Docker {#docker}

[Docker][docker-website]⮳

[Docker Rust guide][docker-rust-guide]⮳

[Docker Desktop][docker-desktop-website]⮳

### References

- Docker's [getting started][docker-getting-started]⮳ docs.{{hi:docker}}
- [Docker's Rust guide][docker-rust-guide]⮳

## Docker Compose {#docker-compose}

[Docker Compose][docker-compose-website]⮳

An easy way to get started is to run `docker init` in a new folder, select `Rust` as the project type, then edit the provided `compose.yaml` and `Dockerfile`.{{hi:docker compose}}

```bash
cargo init .
docker init
```

- Build your image: `docker build -t myapp .`.
- If your cloud{{hi:Cloud}} uses a different CPU architecture than your development machine (e.g., you are on a Mac M1 and your cloud provider is amd64), you'll want to build the image for that platform, e.g.: `docker build --platform=linux/amd64 -t myapp .`.
- Start your application by running: `docker compose up --build`

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P1 review](https://github.com/john-cd/rust_howto/issues/988)
</div>
