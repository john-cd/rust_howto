# Dev Container and Docker

{{#include dev_container_and_docker.incl.md}}

The `.devcontainer/Dockerfile` uses a multi-stage build{{hi:Multi-stage build}} to optimize the image size and build time.

## Multi-stage Build Strategy {#multi-stage-build-strategy}

The `Dockerfile` is organized into three main stages:

1.  **`base` stage**: Contains all the system dependencies, common Rust tools (like `clippy`, `rustfmt`), and specialized tools (like `mdbook`, `pandoc`, `tectonic`). It serves as the foundation for the other stages.
2.  **`development` stage**: Extends the `base` stage with additional tools for local development, such as `jq`, `fzf`, the GitHub CLI (`gh`), and debugging tools like `bacon` and `kani-verifier`. It is used by [Dev Container][ex~contributing~using-vs-code]↗ and contains user-specific configuration (like Git settings).
3.  **`ci` stage**: Also extends the `base` stage. It is optimized for building the book in Continuous Integration environments (like GitHub Actions). It copies the entire repository into the image and uses a specific entrypoint script to build the documentation.

The `development` target is used by `.devcontainer/devcontainer.json` to install [`mdbook`][c~mdbook~docs]↗{{hi:mdbook}} and rust tooling{{hi:Rust tooling}}.

## Rust and Docker {#rust-and-docker}

The project uses several tools and techniques to optimize the Rust development experience within Docker:

-   **`cargo binstall`**: To speed up the image build process, we use [`cargo-binstall`][c~cargo-binstall~repo]↗ to install Rust tools as pre-compiled binaries whenever possible, rather than compiling them from source.
-   **`sccache`**: We use [`sccache`][c~sccache~repo]↗ (Shared Compilation Cache) to cache Rust compilation artifacts. This significantly speeds up subsequent builds by avoiding redundant compilation.

If you don't want to use Dev Container{{hi:Dev Container}}, use the following from the project's root directory to manually build the [`docker`][docker~website]↗{{hi:docker}} image and run it.

```bash
docker build --file .devcontainer/Dockerfile --target development --tag rust_howto_dev --build-arg RUST_IMAGE_LABEL=1.88.0-slim-trixie --build-arg MDBOOK_VERSION=0.4.49 .
docker run --rm --detach --name rust_howto_dev1 --volume $(pwd):/code rust_howto_dev
docker exec -it rust_howto_dev1 bash
```

To cache the crate and the target folders from run to run, add

```bash
--mount type=volume,src=rust_howto_cargo_crate_cache,dst=/usr/local/cargo/registry/
--mount type=volume,src=rust_howto_cargo_target_cache,dst=/code/target/
```

To connect to the (host OS) docker engine from within the container, add

```bash
--mount type=bind,src=/var/run/docker.sock,dst=/var/run/docker-host.sock
```

## Docker Compose {#docker-compose}

Test the docker compose{{hi:docker compose}} setup used during development (which Dev Container runs) with:

```bash
cd ./.devcontainer
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
docker build --file .devcontainer/Dockerfile --target ci --tag rust_howto_ci --build-arg RUST_IMAGE_LABEL=1.88.0-slim-trixie --build-arg MDBOOK_VERSION=0.4.49 .
docker run -it --rm --name rust_howto_ci1 --volume $(pwd)/book:/code/bk/book rust_howto_ci bash
```

- Related [StackOverflow][stackoverflow~use-local-dockerfile-in-a-github-action]↗ question.

### Push Image to Docker Hub {#push-image-to-docker-hub}

From the project root folder, use the following to build and push the `development` image:

```bash
docker build --file .devcontainer/Dockerfile --target development --tag johncd/rust_howto_dev:latest --build-arg RUST_IMAGE_LABEL=1.88.0-slim-trixie --build-arg MDBOOK_VERSION=0.4.49 .
# Or `docker tag rust_howto_dev johncd/rust_howto_dev:latest`
docker login
# Or `docker login -u "user" -p "password" docker.io`
docker push johncd/rust_howto_dev:latest
```

Use the following to build and push the CI image:

```bash
docker build --file .devcontainer/Dockerfile --target ci --tag johncd/rust_howto_ci --build-arg RUST_IMAGE_LABEL=1.88.0-slim-trixie --build-arg MDBOOK_VERSION=0.4.49 .
docker login
docker push johncd/rust_howto_ci:latest
```

## Related Topics {#related-topics .skip}

- [[development_environment_setup | Development Environment Setup]].
- [[repository_structure | Repository Structure]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
