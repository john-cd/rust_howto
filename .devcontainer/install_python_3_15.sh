#!/usr/bin/env bash
# TODO Reviewed ??-??-2026

# Install the pinned CPython release used by the Windows setup and dev container.

set -euo pipefail

: "${PYTHON_VERSION:?Set PYTHON_VERSION from .devcontainer/.env}"
: "${PYTHON_LINUX_SOURCE_SHA256:?Set PYTHON_LINUX_SOURCE_SHA256 from .devcontainer/.env}"

python_executable="/usr/local/bin/python${PYTHON_VERSION%.*}"
python_archive="Python-${PYTHON_VERSION}.tar.xz"

as_root() {
    if [[ "$(id -u)" -eq 0 ]]; then
        "$@"
    else
        sudo "$@"
    fi
}

configure_python_commands() {
    printf '%s\n' /usr/local/lib | as_root tee /etc/ld.so.conf.d/python3.15.conf >/dev/null
    as_root ldconfig
    as_root ln -sfn "$python_executable" /usr/local/bin/python3
    as_root ln -sfn "$python_executable" /usr/local/bin/python
    as_root ln -sfn "${python_executable}-config" /usr/local/bin/python3-config
    as_root ln -sfn "${python_executable}-config" /usr/local/bin/python-config
}

if [[ -x "$python_executable" ]] && "$python_executable" --version | grep -Fqx "Python ${PYTHON_VERSION}"; then
    configure_python_commands
    python3 --version
    exit 0
fi

build_directory="$(mktemp -d "${TMPDIR:-/tmp}/python-build.XXXXXX")"
trap 'rm -rf -- "$build_directory"' EXIT

curl --fail --location --proto '=https' --tlsv1.2 --retry 3 \
    "https://www.python.org/ftp/python/${PYTHON_VERSION}/${python_archive}" \
    --output "${build_directory}/${python_archive}"
printf '%s  %s\n' "$PYTHON_LINUX_SOURCE_SHA256" "${build_directory}/${python_archive}" | sha256sum --check --status
tar --extract --xz --file "${build_directory}/${python_archive}" --directory "$build_directory"

(
    cd "${build_directory}/Python-${PYTHON_VERSION}"
    ./configure --prefix=/usr/local --enable-shared --with-ensurepip=install
    make -j"${PYTHON_BUILD_JOBS:-2}"
    as_root make altinstall
)

configure_python_commands
python_version_output="$(python3 --version)"
if [[ "$python_version_output" != "Python ${PYTHON_VERSION}" ]]; then
    printf 'Expected Python %s, got: %s\n' "$PYTHON_VERSION" "$python_version_output" >&2
    exit 1
fi
printf '%s\n' "$python_version_output"
