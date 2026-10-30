<!-- TODO Reviewed ??-??-2026 -->

# Dev Container and Docker

{{#include dev_container_and_docker.incl.md}}

The `.devcontainer/Dockerfile` uses a multi-stage build{{hi:Multi-stage build}} to optimize the image size and build time.

## Multi-stage Build Strategy {#multi-stage-build-strategy}

The `Dockerfile` is organized into three main stages:

1. **`base` stage**: Contains all the system dependencies, common Rust tools (like `clippy`, `rustfmt`), and specialized tools (like `mdbook`, `pandoc`, `tectonic`). It serves as the foundation for the other stages.
2. **`development` stage**: Extends the `base` stage with additional tools for local development, such as `jq`, `fzf`, and the GitHub CLI (`gh`). It includes [Bacon][bacon~repo]↗, a background file checker, and [Kani][kani~repo]↗, a Rust verifier for checking program properties. It is used by [Dev Container][ex~contributing~using-vs-code]↗ and contains user-specific configuration (like Git settings).
3. **`ci` stage**: Also extends the `base` stage. It is optimized for building the book in Continuous Integration environments (like GitHub Actions). It copies the entire repository into the image and uses a specific entrypoint script to build the documentation.

The `development` target is used by `.devcontainer/devcontainer.json` to install [`mdbook`][c~mdbook~docs]↗{{hi:mdbook}} and rust tooling{{hi:Rust tooling}}.

## Rust and Docker {#rust-and-docker}

The project uses several tools and techniques to optimize the Rust development experience within Docker:

- **`cargo binstall`**: To speed up the image build process, we use [`cargo-binstall`][c~cargo-binstall~repo]↗ to install Rust tools as pre-compiled binaries whenever possible, rather than compiling them from source.
- **`sccache`**: We use [`sccache`][c~sccache~repo]↗ (Shared Compilation Cache) to cache Rust compilation artifacts. This significantly speeds up subsequent builds by avoiding redundant compilation.

For background on when a separate Rust compiler cache helps, see [Why You Need `sccache`][blog~why_you_need_sccache]↗.

The local tool workspace depends on the private `john-cd/book_tooling` repository. Provide a GitHub token with read access to that repository as `GH_TOKEN` when building the image. BuildKit mounts it only while Cargo fetches the dependency; do not store it in `.devcontainer/.env`.

To run the development container manually, use Compose from `.devcontainer`. This reads the pinned tool versions from `.env` and applies the development volume configuration:

```bash
cd ./.devcontainer
GH_TOKEN="$(gh auth token --hostname github.com)" docker compose up --build -d
docker compose exec book bash
```

The Compose configuration keeps Cargo's registry, direct Cargo target directory, and `just`/`sccache` caches in named Docker volumes. The latter is mounted at `/root/.cache`; `just` stores per-checkout target and intermediate build outputs below `/root/.cache/rust_howto`, while `sccache` uses `/root/.cache/sccache`. Volume names include the Rust toolchain version so caches are reused only with a matching toolchain.

To connect to the (host OS) docker [engine][cat~rendering::engine]↗{{hi:rendering::engine}} from within the container, add

```bash
--mount type=bind,src=/var/run/docker.sock,dst=/var/run/docker-host.sock
```

## Base Image and Compiler Cache Decisions {#base-image-and-cache-decisions}

- **Base image:** the images start from the official `rust:<version>-slim-trixie` image, which is already the slim Debian variant, and install only the native libraries the examples need. Alpine (musl) is not used: prebuilt dependencies such as ONNX Runtime and Oracle Instant Client, and the GTK, WebKitGTK and audio libraries, target glibc, and some crates behave differently on musl. Switch only if a measurement shows a benefit in size, build time or caching that outweighs those costs; none has been shown.
- **`sccache`:** the compiler cache is installed in the image but opt-in: set `RUSTC_WRAPPER=sccache`. Measurements on a 20-core Windows machine showed that edit-and-rebuild in a warm target directory takes 5-8 s without it, and that `sccache` reaches a 96 % hit rate only when the same directories are rebuilt after `cargo clean` (about 58 % in fresh target directories, because proc-macro dependents miss). The larger wins came from `debug = "line-tables-only"` with `debug = 0` for dependencies, which halved a cold build (196 s to 90 s) and the size of the target directory. In the container, keep the Cargo registry and `/root/.cache` in named volumes (see above) and enable `sccache` only for throwaway checkouts or CI jobs that start from empty target directories.
- **Pre-commit hooks:** `prek` is installed in the image; the hooks themselves are installed per checkout with `just precommit setup`, so nothing else belongs in the Dockerfile.

## Docker Compose {#docker-compose}

Test the docker compose{{hi:docker compose}} setup used during development (which Dev Container runs) with:

```bash
cd ./.devcontainer
docker compose config --quiet
docker compose build # Uses `compose.yaml` and `compose.override.yaml`
docker compose up -d
# Or simply
docker compose up --build -d
```

## Deploy to GitHub Pages {#deployment-to-github-pages}

The continuous integration workflow{{hi:Continuous integration workflow}} is found under `.github`{{hi:.github}}. See the [GitHub Actions documentation][github-actions~website]↗ for details.

Test the docker compose setup used during CI using:

```bash
cd ./.devcontainer
docker compose -f compose.yaml -f compose-ci.yaml build
docker compose -f compose.yaml -f compose-ci.yaml run book # Or simply docker compose -f compose.yaml -f compose-ci.yaml up
```

It uses the `ci` target in `.devcontainer/Dockerfile`.

To test the [`docker`][docker~website]↗{{hi:docker}} image manually, use

```bash
docker build --file .devcontainer/Dockerfile --target ci --tag rust_howto_ci --build-arg RUST_IMAGE_LABEL=1.99.1-slim-trixie --build-arg MDBOOK_VERSION=0.5.4 .
docker run -it --rm --name rust_howto_ci1 --volume $(pwd)/book:/code/bk/book rust_howto_ci bash
```

- Related [StackOverflow][stackoverflow~use-local-dockerfile-in-a-github-action]↗ question.

### Push Image to Docker Hub {#push-image-to-docker-hub}

From the project root folder, use the following to build and push the `development` image:

```bash
docker build --file .devcontainer/Dockerfile --target development --tag johncd/rust_howto_dev:latest --build-arg RUST_IMAGE_LABEL=1.99.1-slim-trixie --build-arg MDBOOK_VERSION=0.5.4 .
# Or `docker tag rust_howto_dev johncd/rust_howto_dev:latest`
docker login
# Or `docker login -u "user" -p "password" docker.io`
docker push johncd/rust_howto_dev:latest
```

Use the following to build and push the CI image:

```bash
docker build --file .devcontainer/Dockerfile --target ci --tag johncd/rust_howto_ci --build-arg RUST_IMAGE_LABEL=1.99.1-slim-trixie --build-arg MDBOOK_VERSION=0.5.4 .
docker login
docker push johncd/rust_howto_ci:latest
```

## Related Topics {#related-topics .skip}

- [[development_environment_setup | Development Environment Setup]].
- [[repository_structure | Repository Structure]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  accuracy review: contributing guide
</div>
