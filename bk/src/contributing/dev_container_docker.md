# Dev Container and Docker

{{#include dev_container_docker.incl.md}}

The `development` target of the multi-stage `.devcontainer\Dockerfile` is used by `.devcontainer/devcontainer.json` to install [`mdbook`][c~mdbook~docs]{{hi:mdbook}}↗ and rust tooling{{hi:Rust tooling}}.

If you don't want to use Dev Container{{hi:Dev Container}}, use the following from the project's root directory to manually build the [`docker`][docker~website]{{hi:docker}}↗ image and run it.

```bash
docker build --file .devcontainer/Dockerfile --target development --tag rust_howto_dev --build-arg RUST_IMAGE_LABEL=1.75.0-slim-bookworm --build-arg MDBOOK_VERSION=0.4.36 .
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

## Deployment to GitHub Pages {#deployment-to-github-pages}

The continuous integration workflow{{hi:Continuous integration workflow}} is found under `.github`.

Test the docker compose setup used during CI using:

```bash
cd ./.devcontainer
docker compose -f compose.yaml -f compose-ci.yaml build
docker compose -f compose.yaml -f compose-ci.yaml run book # Or simply docker compose -f compose.yaml -f compose-ci.yaml up
```

It uses the `ci` target in `.devcontainer/Dockerfile`.

To test the [`docker`][docker~website]{{hi:docker}}↗ image manually, use

```bash
docker build --file .devcontainer/Dockerfile --target ci --tag rust_howto_ci --build-arg RUST_IMAGE_LABEL=1.75.0-slim-bookworm --build-arg MDBOOK_VERSION=0.4.36 .
docker run -it --rm --name rust_howto_ci1 --volume $(pwd)/book:/code/bk/book rust_howto_ci bash
```

[Related Stackoverflow question][stackoverflow~use-local-dockerfile-in-a~github-action]↗.

### Push Image to Docker Hub {#push-image-to-docker-hub}

From the project root folder, use the following to build and push the `development` image:

```bash
docker build --file .devcontainer/Dockerfile --target development --tag johncd/rust_howto_dev:latest --build-arg RUST_IMAGE_LABEL=1.75.0-slim-bookworm --build-arg MDBOOK_VERSION=0.4.36 .
# Or `docker tag rust_howto_dev johncd/rust_howto_dev:latest`
docker login
# Or `docker login -u "user" -p "password" docker.io`
docker push johncd/rust_howto_dev:latest
```

Use the following to build and push the CI image:

```bash
docker build --file .devcontainer/Dockerfile --target ci --tag johncd/rust_howto_ci --build-arg RUST_IMAGE_LABEL=1.75.0-slim-bookworm --build-arg MDBOOK_VERSION=0.4.36 .
docker login
docker push johncd/rust_howto_ci:latest
```

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[dev_container_docker: review; rust and Docker; multistage builds. NOW](https://github.com/john-cd/rust_howto/issues/525)
</div>
