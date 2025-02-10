set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := justfile_directory()

_default:
  @just --list docker --unsorted --justfile {{justfile()}}
#@just --choose

# Build the CI Docker image, then push it to DockerHub.
[group('push')]
[unix]
push_ci:
  #! /bin/bash
  set -euo pipefail
  ## Access private images in DockerHub
  docker login
  d="{{root}}/.devcontainer/"
  cd ${d}
  docker buildx bake -f compose.yaml -f compose-ci.yaml --pull --push --allow=fs.read=..
  ## OR: docker compose -f compose.yaml -f compose-ci.yaml build --pull --push

# Push the development Docker image to DockerHub.
[group('push')]
@push_dev:
  docker push johncd/rust_howto_dev:latest

# List containers / Docker Compose services
[unix]
list_services:
  #! /bin/bash
  set -euo pipefail
  d="{{root}}/.devcontainer/"
  docker compose -f ${d}compose.yaml \
    -f ${d}compose.override.yaml \
    -f ${d}compose-heavy-tests.yaml \
    ps --all

#[unix]
#down:
#  #! /bin/bash
#  set -euo pipefail
#  d="{{root}}/.devcontainer/"
#  docker compose -f ${d}compose.yaml \
#    -f ${d}compose.override.yaml \
#    -f ${d}compose-heavy-tests.yaml down

# List docker networks
#list_networks:
#  docker network ls

# Inspect docker default network
#inspect_default_net:
#  docker network inspect rust_howto_dev_default

# Get IP addresses of containers
#[unix]
#get_ip:
#  docker inspect -f '{{.Name}} - {{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' $(docker ps -aq)

# Resolve DNS
#resolve:
#  #apt update && apt-get install -y dnsutils
#  nslookup rust_howto_dev-redis-1
#  #apt install iputils-ping
#  ping rust_howto_dev-redis-1
