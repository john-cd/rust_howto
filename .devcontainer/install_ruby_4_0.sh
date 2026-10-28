#!/usr/bin/env bash
set -euo pipefail

ruby_version=4.0.9
ruby_archive="ruby-${ruby_version}.tar.xz"
ruby_url="https://cache.ruby-lang.org/pub/ruby/4.0/${ruby_archive}"
ruby_sha256=a8057adb39f51ad6dc3106237504c93dd792bb515837b4fe7817ae21ed7e51f4
ruby_bin=/usr/local/bin/ruby

if [[ -x "${ruby_bin}" ]] && "${ruby_bin}" --version | grep -Fq "ruby ${ruby_version}"; then
    "${ruby_bin}" --version
    exit 0
fi

build_directory="$(mktemp -d "${TMPDIR:-/tmp}/ruby-build.XXXXXX")"
trap 'rm -rf -- "${build_directory}"' EXIT

curl --fail --location --retry 3 --output "${build_directory}/${ruby_archive}" "${ruby_url}"
printf '%s  %s\n' "${ruby_sha256}" "${build_directory}/${ruby_archive}" | sha256sum --check --status
tar --extract --xz --file "${build_directory}/${ruby_archive}" --directory "${build_directory}"

(
    cd "${build_directory}/ruby-${ruby_version}"
    ./configure --prefix=/usr/local --enable-shared --disable-install-doc --disable-yjit --disable-zjit
    make -j"${RUBY_BUILD_JOBS:-2}"
    if [[ "$(id -u)" -eq 0 ]]; then
        make install
    else
        sudo make install
    fi
)

if [[ "$(id -u)" -eq 0 ]]; then
    ldconfig
else
    sudo ldconfig
fi

"${ruby_bin}" --version
