#! /bin/bash

## Build the CI Docker image, then push it to DockerHub. The GitHub Action CI workflow will use the Dockerhub image as a cache.
## Advanced version of what is in `justfile`. Uses a custom builder.

set -e

## Align the Docker builder configuration with that of the GitHub Actions workflow:
## The workflow uses the GitHub Actions cache, which is not supported with the default `docker` build driver.

## 1) Create a custom builder with the `docker-container` driver, select it for use, and start it:
docker buildx create --name=container --driver=docker-container --use --bootstrap

## 2) Log on Docker to access private images
docker login

## 3) Build with the custom builder. The image is pushed to the registry, not stored locally.
## Changing the cwd seems required to read the .env file
cd ./.devcontainer/
docker buildx bake -f compose.yaml -f compose-ci.yaml --builder=container --pull --push --allow=fs.read=..

## 4) Delete the custom builder
docker buildx rm --keep-state container

## Helpers:
## - Display the existing builders
# docker buildx ls
## - Create a custom build driver that behaves in a similar way to the default docker driver, and load images to the local image store by default.
## https://docs.docker.com/build/builders/drivers/
# docker buildx create --name=container --driver=docker-container --use --bootstrap --driver-opt default-load=true
## - Remove the builder while persisting / caching its state:
## https://docs.docker.com/build/builders/drivers/docker-container/
# docker buildx rm --keep-state container
## - Remove all inactive builders
# docker buildx rm --all-inactive -f
## - Print the configuration used by bake
# docker buildx bake -f .devcontainer/compose.yaml -f .devcontainer/compose-ci.yaml --builder=container --print
## - Build directly with the custom driver; use --load to load the image to the local image store and --push to push to DockerHub
# docker buildx build -f .devcontainer/Dockerfile --target ci --tag johncd/rust_howto_ci:latest --builder=container --cache-from=type=registry,ref=johncd/rust_howto_ci:latest --push .
