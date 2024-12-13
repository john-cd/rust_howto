_default:
  @just --list pre-push --unsorted --justfile {{justfile()}}
#@just --choose

# Build the CI Docker image, then push it to DockerHub.
push_ci:
  #! /bin/bash
  ## Access private images in DockerHub
  docker login
  cd {{justfile_directory()}}/.devcontainer/
  docker buildx bake -f compose.yaml -f compose-ci.yaml --pull --push --allow=fs.read=..
  ## OR: docker compose -f {{justfile_directory()}}/.devcontainer/compose.yaml -f {{justfile_directory()}}/.devcontainer/compose-ci.yaml build --pull --push

# Push the development Docker image to DockerHub.
push_dev:
  docker push johncd/rust_howto_dev:latest
