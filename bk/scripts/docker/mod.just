# Purpose: Build and inspect the repository development and CI Docker images.
# Usage: From bk/, run just docker <recipe>.
#
# TODO Reviewed ??-??-2026.

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

root := replace(clean(source_directory() + "/../.."), "\\", "/")

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

# Build the CI Docker image, then push it to DockerHub.
[group('push')]
push_ci:
    #!/usr/bin/env bash
    set -euo pipefail
    ## Access private images in DockerHub
    docker login
    d="{{ root }}/../.devcontainer/"
    cd ${d}
    docker buildx bake -f compose.yaml -f compose-ci.yaml --pull --push --allow=fs.read=..
    ## OR: docker compose -f compose.yaml -f compose-ci.yaml build --pull --push

# Push the development Docker image to DockerHub.
[group('push')]
@push_dev:
    docker push johncd/rust_howto_dev:latest

# List containers / Docker Compose services
list_services:
    #!/usr/bin/env bash
    set -euo pipefail
    d="{{ root }}/.devcontainer/"
    docker compose -f ${d}compose.yaml \
      -f ${d}compose.override.yaml \
      -f ${d}compose-heavy-tests.yaml \
      ps --all

#down:
#  #!/usr/bin/env bash
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
#get_ip:
#  docker inspect -f '{{.Name}} - {{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' $(docker ps -aq)

# Resolve DNS
#resolve:
#  #apt update && apt-get install -y dnsutils
#  nslookup rust_howto_dev-redis-1
#  #apt install iputils-ping
#  ping rust_howto_dev-redis-1
