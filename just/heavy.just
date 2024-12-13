_default:
  @just --list heavy --unsorted --justfile {{justfile()}}
#@just --choose

## ---- HEAVY TESTS -----------------------------------

# Run tests that require external services - databases, caches...
test:
  #! /bin/bash
  ## $feature is both the name of the feature in `deps/Cargo.toml`
  ## and the service in `compose-heavy-tests.yaml`
  for feature in redis # postgres # "elasticsearch" "meilisearch" "mongodb" "postgres" "redis" "typesense"
  do
    echo ">> $feature"
    ## Start a service, waiting for it to be running|healthy.
    docker compose -f {{justfile_directory()}}/.devcontainer/compose.yaml -f {{justfile_directory()}}/.devcontainer/compose.override.yaml \
      -f {{justfile_directory()}}/.devcontainer/compose-heavy-tests.yaml up --wait ${feature}
    ## Run tests, which names include the feature name, in the `deps` crate only, with the desired compile feature enabled
    cargo nextest run --package dependencies --tests --locked --no-tests=warn --no-fail-fast --success-output final --features $feature -- $feature
    ## Removes stopped service containers. Stop the containers, if required, before removing
    docker compose -f {{justfile_directory()}}/.devcontainer/compose.yaml -f {{justfile_directory()}}/.devcontainer/compose.override.yaml \
      -f {{justfile_directory()}}/.devcontainer/compose-heavy-tests.yaml rm -f --stop --volumes $feature
    echo "------------------------"
  done

# Test the Redis connection:
test_redis_connection:
  docker compose -f  {{justfile_directory()}}/.devcontainer/compose.yaml -f {{justfile_directory()}}/.devcontainer/compose.override.yaml \
    -f {{justfile_directory()}}/.devcontainer/compose-heavy-tests.yaml up --wait redis
  docker run -it --rm --network rust_howto_dev_default --name redis-cli redis:alpine redis-cli -h rust_howto_dev-redis-1 -p 6379

# List containers / Docker COmpose services
list_services:
  docker compose -f {{justfile_directory()}}/.devcontainer/compose.yaml -f {{justfile_directory()}}/.devcontainer/compose.override.yaml \
    -f {{justfile_directory()}}/.devcontainer/compose-heavy-tests.yaml ps --all

## List networks
# docker network ls

## Inspect default network
# docker network inspect rust_howto_dev_default

## Get IP addresses of containers
# docker inspect -f '{{.Name}} - {{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' $(docker ps -aq)

## Resolve DNS
# apt update && apt-get install -y dnsutils
# nslookup rust_howto_dev-redis-1
# apt install iputils-ping
# ping rust_howto_dev-redis-1
